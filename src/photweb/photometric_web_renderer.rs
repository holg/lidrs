use crate::photweb::PhotometricWeb;
use std::path::Path;
use property::Property;
use std::fs::File;
use plotters::prelude::*;
use plotters::style::RGBColor;

#[derive(Default, Debug, Property)]
#[property(get(public), set(public))]
struct PhotometricWebRenderer {
    // Radius of the chart
    pub chart_radius: i32,
    // Difference between two rings
    pub ring_diff: i32,
    // Number of rings
    pub num_rings: i32,
    // Number of divisions
    pub num_divisions: i32,
    // Radius of the label
    pub label_radius: i32,
    // Font size of the label
    pub label_font_size: i32,
    // Size of the chart
    pub chart_size: u32,
    // Photometric web
    pub photweb: PhotometricWeb,
    // Input file
    pub input_file: Option<Box<Path>>,
}

impl PhotometricWebRenderer {
    /// Creates a new `PhotometricWebRenderer` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `PhotometricWebRenderer` with default values.
    fn default() -> Self {
        Self {
            chart_radius: 200,
            ring_diff: 50,
            num_rings: 5,
            num_divisions: 12,
            label_radius: 220,
            label_font_size: 14,
            chart_size: 500,
            photweb: PhotometricWeb::default(),
            input_file: None,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::io::eulumdat::EulumdatFile;
    use super::*;

    #[test]
    fn test_default() {
        let phot_web_render = PhotometricWebRenderer::new();

        assert_eq!(phot_web_render.chart_radius, 200);
        assert_eq!(phot_web_render.ring_diff, 50);
        assert_eq!(phot_web_render.num_rings, 5);
        assert_eq!(phot_web_render.num_divisions, 12);
        assert_eq!(phot_web_render.label_radius, 220);
        assert_eq!(phot_web_render.label_font_size, 14);
        assert_eq!(phot_web_render.chart_size, 500);
        //assert_eq!(phot_web_render.phetwebrend, PhotometricWeb::default());
        assert_eq!(phot_web_render.input_file, None)
    }

    #[test]
    fn test_render() -> Result<(), Box<dyn std::error::Error>>{
        let mut photwebrend = PhotometricWebRenderer::new();
        let eulumdat = EulumdatFile::parse_file(Path::new("./src/io/eulumdat/lampsets.ldt")).unwrap();
        photwebrend.photweb = eulumdat.clone().into();
        let root = SVGBackend::new("plane.svg", (photwebrend.chart_size, photwebrend.chart_size)).into_drawing_area();
        let mut chart = ChartBuilder::on(&root)
            .caption("Plane Plot", ("sans-serif", 20).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_ranged(-360.0..360.0, -360.0..360.0)?;        // Draw rings
        let ring_width = photwebrend.chart_radius / photwebrend.num_rings;
        for i in 1..=photwebrend.num_rings {
            let r = i * ring_width;
            root.draw(&Circle::new(
                (photwebrend.chart_radius + photwebrend.ring_diff, photwebrend.chart_radius + photwebrend.ring_diff),
                r,
                Into::<ShapeStyle>::into(&BLACK),
            ))?;
            // Add label
            let label_x = photwebrend.label_radius as f32 + 10 as f32;
            let label_y = r as i32;
            let label_text = format!("{}", r);
            root.draw(
                &Text::new(
                    label_text.clone(),
                    (label_x as i32, label_y as i32),
                    FontFamily::SansSerif,
                )
            )?;
            let label_y2 = r * -1 as i32;
            // chart.draw(
            //     &Text::new(
            //         label_text.clone(),
            //         (label_x as i
        }
        // Draw divisions
        let label_radius = photwebrend.chart_radius + 20;
        let division_angle = 360.0 / photwebrend.num_divisions as f32;
        let middle_chart = photwebrend.chart_size as f32  / 2 as f32;
        for i in 0..photwebrend.num_divisions {
            let angle_degrees = i as f32 * division_angle;
            let angle_radians = angle_degrees.to_radians();
            let x1 = middle_chart + (photwebrend.chart_radius - photwebrend.chart_radius) as f32 * angle_radians.cos();
            let y1 = middle_chart - (photwebrend.chart_radius - photwebrend.chart_radius) as f32 * angle_radians.sin();
            let x2 = middle_chart + (photwebrend.chart_radius + 0) as f32 * angle_radians.cos();
            let y2 = middle_chart - (photwebrend.chart_radius + 0) as f32 * angle_radians.sin();
            // Convert LineSeries to Polyline
            let line_coords = vec![(x1 as i32, y1 as i32), (x2 as i32, y2 as i32)];
            // Add labels
            let label_x = middle_chart + label_radius as f32 * angle_radians.cos();
            let label_y = middle_chart - label_radius as f32 * angle_radians.sin();
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
            root.draw(
                &Text::new(
                    label_text,
                    (label_x as i32, label_y as i32),
                    FontFamily::SansSerif,
                )
            )?;
            root.draw(&PathElement::new(line_coords, line_style))?;
        }
        let planes = photwebrend.photweb.planes();

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
}