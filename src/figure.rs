use std::vec::Vec;

#[derive(Clone)]
pub struct FigureData {
    pub position : (i32, i32),
    pub sequence : Vec<(i32, i32)>
}

impl FigureData {
    pub fn new(position : (i32, i32), sequence : Vec<(i32, i32)>) -> FigureData {
        FigureData{position, sequence}
    }

    pub fn get_sequence<'a>(&'a self) -> &'a [(i32, i32)] {
        &self.sequence[..]
    }
}

#[derive(Clone)]
pub struct PreviewFigureData {
    pub figure : FigureData,
    pub offset : (f64, f64)
}

impl PreviewFigureData {
    pub fn initializeFigures() -> Vec<PreviewFigureData> {
        let mut result : Vec<PreviewFigureData> = Vec::with_capacity(7);
        // I
        result.push(PreviewFigureData{figure :FigureData::new(
            (5, 0),
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]),
            offset : (0.0, 0.0)}
        );

        //O
        result.push(PreviewFigureData{figure :FigureData::new(
            (5, 0),
            vec![(0, 0), (1, 0), (0, 1),(1, 1)]),
            offset : (0.0, 0.0)}
        );

        //T
        result.push(PreviewFigureData{figure :FigureData::new(
            (5, 0),
            vec![(0, 0), (1, 0), (2, 0),(1, 1)]),
            offset : (0.0, 0.0)}
        );

        //S
        result.push(PreviewFigureData{figure :FigureData::new(
            (5, 0),
            vec![(0, 0), (1, 0), (0, 1),(-1, 1)]),
            offset : (0.0, 0.0)}
        );

        //Z
        result.push(PreviewFigureData{figure :FigureData::new(
            (5, 0),
            vec![(0, 0), (1, 0), (1, 1),(2, 1)]),
            offset : (0.0, 0.0)}
        );


        //J
        result.push(PreviewFigureData{figure :FigureData::new(
            (5, 0),
            vec![(0, 0), (0, 1), (1, 1),(2, 1)]),
            offset : (0.0, 0.0)}
        );

        //L
        result.push(PreviewFigureData{figure :FigureData::new(
            (5, 0),
            vec![(0, 0), (0, 1), (-1, 1),(-2, 1)]),
            offset : (0.0, 0.0)}
        );
        result
    }
    pub fn figureData(&self) -> FigureData {
        self.figure.clone()
    }
}