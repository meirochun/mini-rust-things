#[warn(unused_assignments)]
fn main() {
    let distancia_total = 1000.0;
    let tempo_total = 10.0;

    let velocidade_media = calc(distancia_total, tempo_total);
    println!("\n\tResposta: {}m/s", velocidade_media.round());
    println!("\tResposta: {}km/h\n\n", velocidade_media.round() * 3.6);
}

fn calc(dt: f64, tt: f64) -> f64 {
    return dt / tt;
}
