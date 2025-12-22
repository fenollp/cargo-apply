fn touched() {
    let _ = 42.type_of("smthg");

    let _ = (g() + 1).type_of(bla);
}

fn untouched() {
    let _ = type_of("bla");

    let _ = 42.type_of();

    let _ = 42.type_of(blip, blop);
}
