use approx::assert_relative_eq;
use feos_core::parameter::{IdentifierOption, Parameter};
use feos_core::{EquationOfState, StateBuilder};
use feos_pcsaft::{PcSaft, PcSaftParameters};
use ndarray::*;
use quantity::si::*;
use std::error::Error;
use std::rc::Rc;

#[test]
fn test_dln_phi_dp() -> Result<(), Box<dyn Error>> {
    let params = PcSaftParameters::from_json(
        &["propane", "butane"],
        "tests/test_parameters.json",
        None,
        IdentifierOption::Name,
    )?;
    let saft = Rc::new(PcSaft::new(params));
    let t = 300.0 * KELVIN;
    let p = BAR;
    let h = 1e-1 * PASCAL;
    let s = StateBuilder::new(&saft)
        .temperature(t)
        .pressure(p)
        .molefracs(&arr1(&[0.5, 0.5]))
        .vapor()
        .build()?;
    let sh = StateBuilder::new(&saft)
        .temperature(t)
        .pressure(p + h)
        .molefracs(&arr1(&[0.5, 0.5]))
        .vapor()
        .build()?;

    let ln_phi = s.ln_phi()[0];
    let ln_phi_h = sh.ln_phi()[0];
    let dln_phi_dp = s.dln_phi_dp().get(0);
    let dln_phi_dp_h = (ln_phi_h - ln_phi) / h;
    assert_relative_eq!(dln_phi_dp, dln_phi_dp_h, max_relative = 1e-6);
    Ok(())
}

#[test]
fn test_virial_is_not_nan() -> Result<(), Box<dyn Error>> {
    let params = PcSaftParameters::from_json(
        &["water_np"],
        "tests/test_parameters.json",
        None,
        IdentifierOption::Name,
    )?;
    let saft = Rc::new(PcSaft::new(params));
    let virial_b = saft.second_virial_coefficient(300.0 * KELVIN, None)?;
    assert!(!virial_b.is_nan());
    Ok(())
}
