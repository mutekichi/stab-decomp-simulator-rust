use crate::StabilizerCHForm;
use ndarray::{Zip, s};

impl StabilizerCHForm {
    pub(crate) fn xor_rows(matrix: &mut ndarray::Array2<bool>, target: usize, source: usize) {
        // Split view to allow simultaneous mutable borrows
        let (mut row_target, row_source) = matrix.multi_slice_mut((s![target, ..], s![source, ..]));

        Zip::from(&mut row_target)
            .and(&row_source)
            .for_each(|t, &s| {
                *t ^= s;
            });
    }

    pub(crate) fn xor_columns(matrix: &mut ndarray::Array2<bool>, target: usize, source: usize) {
        // Split view to allow simultaneous mutable borrows
        let (mut col_target, col_source) = matrix.multi_slice_mut((s![.., target], s![.., source]));

        Zip::from(&mut col_target)
            .and(&col_source)
            .for_each(|t, &s| {
                *t ^= s;
            });
    }
}
