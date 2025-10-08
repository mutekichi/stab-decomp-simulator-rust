use crate::StabilizerCHForm;

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_swap(&mut self, qarg1: usize, qarg2: usize) {
        let perm: Vec<usize> = (0..self.n)
            .map(|x| match x {
                _ if x == qarg1 => qarg2,
                _ if x == qarg2 => qarg1,
                _ => x,
            })
            .collect();
        self._permute(&perm);
    }
}
