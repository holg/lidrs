use super::{EulumdatFile, EulumdatSymmetry};
use crate::photweb::PhotometricWeb;
use approx::assert_relative_eq;
use std::path::Path;
use std::fs::File;
use plotters::prelude::*;
/// Example file provided by Paul Bourne's documentation:
/// http://paulbourke.net/dataformats/ldt/
const EXAMPLE_LDT_FILE_NO_LAMPSET: &str = include_str!("example.ldt");
const EXAMPLE_LDT_FILE_LAMPSET: &str = include_str!("lampsets.ldt");


#[test]
fn draw_grid() -> Result<(), Box<dyn std::error::Error>> {
    // Define chart parameters
    let chart_radius = 200;
    let ring_diff = 50;
    let num_rings = 5;
    let num_divisions = 12;
    let label_radius = chart_radius + 20;
    let label_font_size = 14;
    let chart_size = 500;

    // Create drawing area

    let chart = SVGBackend::new("polar_chart.svg", (chart_size, chart_size)).into_drawing_area();
    // Draw rings
    let ring_width = chart_radius / num_rings;
    for i in 1..=num_rings {
        let r = i * ring_width;
        chart.draw(&Circle::new(
            (chart_radius + ring_diff, chart_radius + ring_diff),
            r,
            Into::<ShapeStyle>::into(&BLACK),
        ))?;
        // Add label
        let label_x =  label_radius as f32 + 10 as f32;
        let label_y = r  as i32;
        let label_text = format!("{}", r);
        chart.draw(
            &Text::new(
                label_text.clone(),
                (label_x as i32, label_y as i32),
                FontFamily::SansSerif,
            )
        )?;
        let label_y2 = r * -1  as i32;
        // chart.draw(
        //     &Text::new(
        //         label_text.clone(),
        //         (label_x as i32, label_y2  as i32),
        //         FontFamily::SansSerif,
        //     )
        // )?;
    }

    // Draw divisions
    let division_angle = 360.0 / num_divisions as f32;
    for i in 0..num_divisions {
        let angle_degrees = i as f32 * division_angle;
        let angle_radians = angle_degrees.to_radians();
        let x1 = 250.0 + (chart_radius - chart_radius) as f32 * angle_radians.cos();
        let y1 = 250.0 - (chart_radius - chart_radius) as f32 * angle_radians.sin();
        let x2 = 250.0 + (chart_radius + 0) as f32 * angle_radians.cos();
        let y2 = 250.0 - (chart_radius + 0) as f32 * angle_radians.sin();
        // Convert LineSeries to Polyline
        let line_coords = vec![(x1 as i32, y1 as i32), (x2 as i32, y2 as i32)];
        // Add labels
        let label_x = 250.0 + label_radius as f32 * angle_radians.cos();
        let label_y = 250.0 - label_radius as f32 * angle_radians.sin();
        let mut label_value = (angle_degrees - 270.0).abs();
        if label_value > 180.0 {
            label_value = 360.0 - label_value;
        }
        let label_text = format!("{:.0}°", label_value);
        let line_style = ShapeStyle {
            color: BLACK.mix(0.6),
            filled: false,
            stroke_width: 1,
        };
        chart.draw(
            &Text::new(
                label_text,
                (label_x as i32, label_y as i32),
                FontFamily::SansSerif,
            )
        )?;
        chart.draw(&PathElement::new(line_coords, line_style))?;


    }
    // Finalize drawing
    chart.present()?;
    Ok(())
}



#[test]
fn test_plot() -> Result<(), Box<dyn std::error::Error>> {
    //let eulumdat = EulumdatFile::parse_file(Path::new("./src/io/eulumdat/example.ldt")).unwrap();
    let eulumdat = EulumdatFile::parse_file(Path::new("./src/io/eulumdat/lampsets.ldt")).unwrap();
    let photweb: PhotometricWeb = eulumdat.clone().into();
    let root = SVGBackend::new("plane.svg", (640, 640)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Plane Plot", ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(-360.0..360.0, -360.0..360.0)?;

    chart.configure_mesh().draw()?;

    let planes = photweb.planes();

    for plane in planes {
        //let test = plane
        let plane_width = plane.width();
        let plane_orientation = plane.orientation();
        let plane_integrate_intensity = plane.integrate_intensity();
        let rotation_angle = 270.0_f64.to_radians();

        let polar_series: Vec<(f64, f64)> = plane
            .angles()
            .iter()
            .zip(plane.intensities().iter())
            .map(|(angle, intensity)| ((angle + rotation_angle).cos() * intensity, (angle+ rotation_angle).sin() * intensity))
            .collect();

        chart.draw_series(LineSeries::new(polar_series, &RED))?;;
    }
    root.present()?;

    Ok(())
}

/*fn test_cartesian_plot() -> Result<(), Box<dyn std::error::Error>> {
    //let eulumdat = EulumdatFile::parse_file(Path::new("./src/io/eulumdat/example.ldt")).unwrap();
    let eulumdat = EulumdatFile::parse_file(Path::new("./src/io/eulumdat/lampsets.ldt")).unwrap();
    let photweb: PhotometricWeb = eulumdat.clone().into();
    let planes = photweb.planes();
    let root = SVGBackend::new("plot.svg", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    let (max_radius, data): (f64, Vec<(f64, f64)>) = (
        1.0,
        vec![
            (1.0, 0.0),
            (0.866, 0.5),
            (0.5, 0.866),
            (0.0, 1.0),
            (-0.5, 0.866),
            (-0.866, 0.5),
            (-1.0, 0.0),
            (-0.866, -0.5),
            (-0.5, -0.866),
            (0.0, -1.0),
            (0.5, -0.866),
            (0.866, -0.5),
        ],
    );

    let mut chart = ChartBuilder::on(&root)
        .caption("A polar plot", ("sans-serif", 40))
        .margin(20)
        .build_ranged(-max_radius..max_radius, -max_radius..max_radius)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart
        .draw_series(
            Map::new(data.into_iter().map(|(x, y)| (x, y)).into_iter())
                .polar_line_style(&RED.mix(0.5))
                .fill_style(&RED.mix(0.2))
                .to_polar_lines(),
        )?
        .label("My data")
        .legend(|(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], RED.filled()));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .font(("sans-serif", 20))
        .draw()?;

    let label_angles: Vec<f64> = (-180..=180).step_by(30).map(|x| (x as f64).to_radians()).collect();
    chart
        .configure_mesh()
        .set_labels_custom(&label_angles, |l| format!("{:.0}°", (l.to_degrees())))
        .draw()?;

    let data = [(0_f64, 0.2), (45_f64, 0.4), (90_f64, 0.6), (135_f64, 0.8), (180_f64, 1.0)];

    chart
        .draw_series(
            data.iter()
                .map(|(angle, radius)| {
                    let angle = angle.to_radians();
                    let end_point = (angle.cos() * radius, angle.sin() * radius);
                    Polygon::new(
                        vec![(0.0, 0.0), end_point],
                        ShapeStyle::from(&BLACK).filled(),
                    )
                })
        )?
        .label("Series 1")
        .legend(|(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &BLACK));

    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).draw()?;

    for plane in planes {
        //let test = plane
        let plane_width = plane.width();
        let plane_orientation = plane.orientation();
        let plane_integrate_intensity = plane.integrate_intensity();
        let rotation_angle = 270.0_f64.to_radians();

        let polar_series: Vec<(f64, f64)> = plane
            .angles()
            .iter()
            .zip(plane.intensities().iter())
            .map(|(angle, intensity)| ((angle + rotation_angle).cos() * intensity, (angle+ rotation_angle).sin() * intensity))
            .collect();
        let path = PathElement::new(polar_series.iter().map(|&(r, theta)| {
            (r * theta.cos(), r * theta.sin())
        }), &RED);

        chart.draw_series(std::iter::once(path))?;

    }
    // Save the plot as an SVG file
    Ok(())
}*/

#[test]
fn test_parse_ldt_example_no_lampset(){
    test_parse_ldt(EXAMPLE_LDT_FILE_NO_LAMPSET);
}

#[test]
fn test_parse_ldt_example_lampset(){
    test_parse_ldt(EXAMPLE_LDT_FILE_LAMPSET);
}

fn test_parse_ldt(ldt_string: &str) {
    let mut ldt = EulumdatFile::new();
    match ldt.parse(&ldt_string.to_owned()) {
        Ok(_) => {
            // Check that the arrays are the correct length.
            assert_eq!(ldt.c_angles().iter().count(), ldt.n_cplanes());
            assert_eq!(
                ldt.g_angles().iter().count(),
                ldt.n_luminous_intensities_per_cplane()
            );
            assert_eq!(
                ldt.get_planes().iter().count(),
                (ldt.mc2() - ldt.mc1() + 1) * ldt.n_luminous_intensities_per_cplane()
            );
        }
        Err(e) => assert!(false, "LDT parse error: {}", e),
    }
}

#[test]
fn test_parse_ldt_file() {
    match EulumdatFile::parse_file(Path::new("./src/io/eulumdat/example.ldt")) {
        Ok(ldt) => {
            // Check that the arrays are the correct length.
            assert_eq!(ldt.c_angles().iter().count(), ldt.n_cplanes());
            assert_eq!(
                ldt.g_angles().iter().count(),
                ldt.n_luminous_intensities_per_cplane()
            );
            assert_eq!(
                ldt.intensities().iter().count(),
                (ldt.mc2() - ldt.mc1() + 1) * ldt.n_luminous_intensities_per_cplane()
            );

        }
        Err(e) => assert!(false, "LDT file parse error: {}", e),
    }
}

#[test]
fn test_ldt_into_photweb() {
    let mut ldt = EulumdatFile::new();
    // match ldt.parse(&EXAMPLE_LDT_FILE_NO_LAMPSET.to_owned()) {
    match ldt.parse(&EXAMPLE_LDT_FILE_LAMPSET.to_owned()) {
        Ok(_) => {
            // Now attempt to convert to a photometric web.
            let photweb: PhotometricWeb = ldt.clone().into();
            let total_intensity = photweb.total_intensity();

            // Test that the parameters have made it across.
            assert_eq!(photweb.planes().iter().count() as usize, ldt.n_cplanes());
        }
        Err(e) => assert!(false, "LDT parse error: {}", e),
    }
}

/// In this test I will be testing that the reconcilliation of symmetry in the photometric web
/// is correct and behaves as we expect for symmetry around the C0-180 C-planes.
#[test]
fn test_get_planes_c0c180_symmetry() {
    let mut ldt = EulumdatFile::new();
    ldt.set_n_cplanes(18 as usize);
    ldt.set_c_angles(
        (0..190)
            .step_by(10)
            .map(|ang| ang as f64)
            .collect::<Vec<f64>>(),
    );
    ldt.set_g_angles(vec![0.0]);
    ldt.set_n_luminous_intensities_per_cplane(1 as usize);
    ldt.set_intensities(
        (0..190)
            .step_by(10)
            .map(|ang| ang as f64)
            .collect::<Vec<f64>>(),
    );
    ldt.set_symmetry(EulumdatSymmetry::C0C180Plane);

    // Perform the conversion.
    let photweb: PhotometricWeb = ldt.clone().into();

    // Check that we have the correct number of angles.
    assert_eq!(photweb.planes().iter().count(), 36);

    // Check that all of the planes are filled with the correct angle.
    let _ = vec![
        0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0, 130.0,
        140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0, 210.0, 220.0, 230.0, 240.0, 250.0, 260.0,
        270.0, 280.0, 290.0, 300.0, 310.0, 320.0, 330.0, 340.0, 350.0,
    ]
        .iter()
        .zip(photweb.planes())
        .map(|(test, pl)| assert_relative_eq!(pl.angle_deg(), test, epsilon = 1E-6))
        .collect::<Vec<_>>();

    // Check that the angles have ended up where we expect them to.
    assert_eq!(
        photweb
            .planes()
            .iter()
            .map(|pl| pl.intensities()[0])
            .collect::<Vec<f64>>(),
        vec![
            0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0, 130.0,
            140.0, 150.0, 160.0, 170.0, 180.0, 170.0, 160.0, 150.0, 140.0, 130.0, 120.0, 110.0,
            100.0, 90.0, 80.0, 70.0, 60.0, 50.0, 40.0, 30.0, 20.0, 10.0
        ]
    );
}

/// Tests the conversation to photometric web for the C90 - C270 plane case.
#[test]
fn test_get_planes_c90c270_symmetry() {
    let mut ldt = EulumdatFile::new();
    ldt.set_n_cplanes(18 as usize);
    ldt.set_c_angles(
        (90..280)
            .step_by(10)
            .map(|ang| ang as f64)
            .collect::<Vec<f64>>(),
    );
    ldt.set_g_angles(vec![0.0]);
    ldt.set_n_luminous_intensities_per_cplane(1 as usize);
    ldt.set_intensities(
        (90..280)
            .step_by(10)
            .map(|ang| ang as f64)
            .collect::<Vec<f64>>(),
    );
    ldt.set_symmetry(EulumdatSymmetry::C90C270Plane);
    // Perform the conversation.
    let photweb: PhotometricWeb = ldt.clone().into();

    // Check that we have the correct number of angles.
    assert_eq!(photweb.planes().iter().count(), 36);
    // Check that all of the planes are filled with the correct angle.
    let _ = vec![
        0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0, 130.0,
        140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0, 210.0, 220.0, 230.0, 240.0, 250.0, 260.0,
        270.0, 280.0, 290.0, 300.0, 310.0, 320.0, 330.0, 340.0, 350.0,
    ]
        .iter()
        .zip(photweb.planes())
        .map(|(test, pl)| assert_relative_eq!(pl.angle_deg(), test, epsilon = 1E-6))
        .collect::<Vec<_>>();

    // Check that the angles have ended up where we expect them to.
    assert_eq!(
        photweb
            .planes()
            .iter()
            .map(|pl| pl.intensities()[0])
            .collect::<Vec<f64>>(),
        vec![
            180.0, 170.0, 160.0, 150.0, 140.0, 130.0, 120.0, 110.0, 100.0, 90.0, 100.0, 110.0,
            120.0, 130.0, 140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0, 210.0, 220.0, 230.0,
            240.0, 250.0, 260.0, 270.0, 260.0, 250.0, 240.0, 230.0, 220.0, 210.0, 200.0, 190.0,
        ]
    );
}

/// Tests the conversation to photometric web for the C90 - C270 plane case.
#[test]
fn test_get_planes_c0c180c90c270_symmetry() {
    let mut ldt = EulumdatFile::new();
    ldt.set_n_cplanes(9 as usize);
    ldt.set_c_angles(
        (0..100)
            .step_by(10)
            .map(|ang| ang as f64)
            .collect::<Vec<f64>>(),
    );
    ldt.set_g_angles(vec![0.0]);
    ldt.set_n_luminous_intensities_per_cplane(1 as usize);
    ldt.set_intensities(
        (0..100)
            .step_by(10)
            .map(|ang| ang as f64)
            .collect::<Vec<f64>>(),
    );
    ldt.set_symmetry(EulumdatSymmetry::C0C180C90C270Plane);

    // Perform the conversion.
    let photweb: PhotometricWeb = ldt.clone().into();

    // Check that we have the correct number of angles.
    assert_eq!(photweb.planes().iter().count(), 36);

    let _ = vec![
        0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0, 130.0,
        140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0, 210.0, 220.0, 230.0, 240.0, 250.0, 260.0,
        270.0, 280.0, 290.0, 300.0, 310.0, 320.0, 330.0, 340.0, 350.0,
    ]
        .iter()
        .zip(photweb.planes())
        .map(|(test, pl)| assert_relative_eq!(pl.angle_deg(), test, epsilon = 1E-6))
        .collect::<Vec<_>>();

    // Check that the angles have ended up where we expect them to.
    assert_eq!(
        photweb
            .planes()
            .iter()
            .map(|pl| pl.intensities()[0])
            .collect::<Vec<f64>>(),
        vec![
            0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 80.0, 70.0, 60.0, 50.0,
            40.0, 30.0, 20.0, 10.0, 0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0,
            80.0, 70.0, 60.0, 50.0, 40.0, 30.0, 20.0, 10.0
        ]
    );
}

/// Check that in the case of spherical symmetry, we only end up with a single plane.
#[test]
fn test_get_planes_spherical_symmetry() {
    let mut ldt = EulumdatFile::new();
    ldt.set_n_cplanes(1 as usize);
    ldt.set_c_angles(vec![0.0]);
    ldt.set_g_angles(vec![0.0]);
    ldt.set_n_luminous_intensities_per_cplane(1 as usize);
    ldt.set_intensities(vec![1.0]);
    ldt.set_symmetry(EulumdatSymmetry::AboutVerticalAxis);

    // Perform the conversion.
    let photweb: PhotometricWeb = ldt.clone().into();

    // Check that we have the correct number of angles.
    assert_eq!(photweb.planes().iter().count(), 1);
    assert_eq!(
        photweb
            .planes()
            .iter()
            .map(|pl| pl.angle())
            .collect::<Vec<f64>>(),
        vec![0.0]
    );
    // Check that the angles have ended up where we expect them to.
    assert_eq!(
        photweb
            .planes()
            .iter()
            .map(|pl| pl.intensities()[0])
            .collect::<Vec<f64>>(),
        vec![1.0]
    );
}
