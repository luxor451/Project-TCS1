use std::fmt::Display;

pub mod selection {
    pub fn sort(arr: &mut [i64]) {}
}

pub mod merge {
    pub fn sort(arr: &mut [i64], aux: &mut [i64]) {
        let len = arr.len();
        assert_eq!(len, aux.len());
        if len < 2 {
            return;
        }
        let (arr_left, arr_right) = arr.split_at_mut(len / 2);
        let (aux_left, aux_right) = aux.split_at_mut(len / 2);
        sort(arr_left, aux_left);
        sort(arr_right, aux_right);
        let mut il = 0;
        let mut ir = 0;
        for i in 0..len {
            let cond = arr_left.get(il).map_or(false, |vil| {
                arr_right.get(ir).map_or(true, |vir| vil <= vir)
            });
            if cond {
                aux[i] = arr_left[il];
                il += 1;
            } else {
                aux[i] = arr_right[ir];
                ir += 1;
            }
        }
        for i in 0..len {
            arr[i] = aux[i]
        }
    }
}
