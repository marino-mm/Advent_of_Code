use std::{
    collections::{BTreeSet, BinaryHeap, HashSet},
    time::Instant,
};

fn main() {
    let start = Instant::now();
    //     let data = "162,817,812
    // 57,618,57
    // 906,360,560
    // 592,479,940
    // 352,342,300
    // 466,668,158
    // 542,29,236
    // 431,825,988
    // 739,650,466
    // 52,470,668
    // 216,146,977
    // 819,987,18
    // 117,168,530
    // 805,96,715
    // 346,949,466
    // 970,615,88
    // 941,993,340
    // 862,61,35
    // 984,92,344
    // 425,690,689";

    let data = include_str!("input.txt");

    let res = solve(&data);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
    println!("Result: {:?}", res);
}

fn solve(data: &str) -> u128 {
    let mut junct_boxes = BTreeSet::<JunctBox>::new();
    let mut dists = Vec::<(JunctBox, JunctBox, f64)>::new();

    let mut circuits = Vec::<HashSet<JunctBox>>::new();

    for line in data.lines() {
        let parts: Vec<u128> = line
            .trim()
            .split(",")
            .map(|p| p.parse::<u128>().unwrap())
            .collect();
        junct_boxes.insert(JunctBox {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        });
    }

    for (jb_1_i, jb_1) in junct_boxes.iter().enumerate() {
        for (jb_2_i, jb_2) in junct_boxes.iter().enumerate() {
            if jb_1_i >= jb_2_i {
                continue;
            }
            if jb_1_i != jb_2_i {
                let dist = get_dist(jb_1, jb_2);
                dists.push((*jb_1, *jb_2, dist));
            }
        }
    }
    dists.sort_by(|a, b| a.2.total_cmp(&b.2));

    for dist in &dists[0..1000] {
        let jb_1 = dist.0;
        let jb_2 = dist.1;

        let mut jb_1_circuit = get_circuit(jb_1, &mut circuits);
        let jb_2_circuit = get_circuit(jb_2, &mut circuits);

        if jb_1_circuit != jb_2_circuit {
            jb_1_circuit.extend(jb_2_circuit);
        }
        circuits.push(jb_1_circuit);
        // println!("{:?}", circuits);
    }

    let mut len_heap = BinaryHeap::<u128>::new();
    for circuit in circuits {
        len_heap.push(circuit.len() as u128);
    }

    let mut res = 1;
    println!("{:?}", len_heap);
    for _ in 0..3 {
        res = res * len_heap.pop().unwrap()
    }

    res
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
struct JunctBox {
    x: u128,
    y: u128,
    z: u128,
}

fn get_dist(jnt_1: &JunctBox, jnt_2: &JunctBox) -> f64 {
    f64::sqrt(
        (jnt_2.x - jnt_1.x).pow(2) as f64
            + (jnt_2.y - jnt_1.y).pow(2) as f64
            + (jnt_2.z - jnt_1.z).pow(2) as f64
    )
}

fn get_circuit(jb_1: JunctBox, circuits: &mut Vec<HashSet<JunctBox>>) -> HashSet<JunctBox> {
    let found_index = circuits.iter().position(|circuit| circuit.contains(&jb_1));

    if let Some(ind) = found_index {
        return circuits.swap_remove(ind);
    }

    let mut new_hash = HashSet::new();
    new_hash.insert(jb_1);
    new_hash
}
