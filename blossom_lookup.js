const IDX_SIZE = 3;

async function load_data () {
    const response = await fetch('./all_solns.json');
    const data = await response.json();

    const bin_response = await fetch('./all_solns.json.bin');
    const arrayBuffer = await bin_response.arrayBuffer();
    const soln_bytes = new Uint8Array(arrayBuffer);
    console.log('Data loaded!');
    return [data, soln_bytes];
}

const soln_data = load_data();

function decode_idx(idx_bytes) {
    return idx_bytes[0] | (idx_bytes[1] << 8) | (idx_bytes[2] << 16);
}

async function solve(input) {
    const fetched_data = await soln_data;
    let soln_slice = fetched_data[0].solutions[input];
    soln_slice[0] *= IDX_SIZE;
    soln_slice[1] *= IDX_SIZE;
    let solns = [];
    for (let i = soln_slice[0];i < soln_slice[1];i+=IDX_SIZE) {
        solns.push(fetched_data[0].words[decode_idx(fetched_data[1].slice(i, i+IDX_SIZE))]);
    }
    console.log(solns);
}

solve("acdentut");