use plotpy::{Plot, StrError, Surface};

fn main() -> Result<(), StrError> {
    
    let r = &[1.0, 1.0, 1.0];
   
    // pyramid
    let c = &[1.0, -1.0, -10.0];
    let k = &[1.0, 1.0, 1.0];
    //let r = &[1.0, 1.0, 1.0];
   // [(3.0, 2.0), (5.0, 2.0), (5.0, 4.0),(8.0, 4.0), (8.0, 2.0), (10.0, 2.0),(9.0, 4.0), 
        (8.0, 5.0), (7.0, 7.0),(6.0, 7.0), (5.0, 5.0), (4.0, 4.0),(3.0,2.0)]
    let mut pyramids = Surface::new();
    pyramids
        .set_colormap_name("tab20c")
        .draw_superquadric(c, r, k, -180.0, 180.0, 0.0, 90.0, 20, 20)?;

  
    // Generar figura en el plano
    let mut plot = Plot::new();
    plot.add(&pyramids);
       
    // Guardar figura 
    plot.set_equal_axes(true)
        .set_figure_size_points(600.0, 600.0)
        .save("/home/juanh/Descargas/Piramide/piramide.svg")?;
    Ok(())
}