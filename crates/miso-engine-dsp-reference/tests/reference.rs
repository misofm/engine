//! Integration checks proving the independent offline API is consumable without engine kernels.

use miso_engine_dsp_reference::{
    Complex64, IdentityProcessor, ReferenceSignalError, SpectrumError, deterministic_bipolar_noise,
    deterministic_impulse, deterministic_sine, direct_dft_bin, direct_dft_frequency, magnitude_db,
    render_planar_f64,
};

#[test]
fn independent_identity_oracle_round_trips_noise() {
    let input = deterministic_bipolar_noise(2, 64, 7).expect("signal");
    let output = render_planar_f64(&mut IdentityProcessor, &input).expect("render");
    assert_eq!(output, input);
}

#[test]
fn signal_and_spectrum_domains_are_strict() {
    assert_eq!(
        deterministic_impulse(1, 8, 8),
        Err(ReferenceSignalError::InvalidParameter)
    );
    assert_eq!(
        deterministic_sine(1, 8, 0.0, 1.0),
        Err(ReferenceSignalError::InvalidParameter)
    );
    assert_eq!(
        direct_dft_bin(&[f64::NAN], 0),
        Err(SpectrumError::NonFiniteInput)
    );
    assert_eq!(
        direct_dft_frequency(&[0.0], 48_000.0, 24_001.0),
        Err(SpectrumError::InvalidFrequency)
    );
    assert_eq!(
        magnitude_db(Complex64 { re: 1.0, im: 0.0 }, f64::NAN),
        Err(SpectrumError::InvalidFloor)
    );
}

#[test]
fn delayed_delta_and_exact_bin_sine_have_known_spectra() {
    let impulse = deterministic_impulse(1, 16, 3).unwrap();
    for bin in 0..16 {
        let value = direct_dft_bin(impulse.channel(0).unwrap(), bin).unwrap();
        assert!((value.re.hypot(value.im) - 1.0).abs() < 1e-12);
    }
    let sine = deterministic_sine(1, 64, 64.0, 7.0).unwrap();
    let magnitudes = (0..33)
        .map(|bin| {
            let value = direct_dft_bin(sine.channel(0).unwrap(), bin).unwrap();
            value.re.hypot(value.im)
        })
        .collect::<Vec<_>>();
    let peak = magnitudes
        .iter()
        .enumerate()
        .max_by(|left, right| left.1.total_cmp(right.1))
        .map(|(index, _)| index)
        .unwrap();
    assert_eq!(peak, 7);
}
