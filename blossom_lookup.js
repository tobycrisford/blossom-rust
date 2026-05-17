const IDX_SIZE = 3;

soln_data = null;
soln_bytes = null;

async function load_data () {
    const response = await fetch('./all_solns.json');
    const data = await response.json();

    const bin_response = await fetch('./all_solns.json.bin');
    const arrayBuffer = await bin_response.arrayBuffer();
    const soln_bytes = new Uint8Array(arrayBuffer);
    console.log('Data loaded!');
    return [data, soln_bytes];
}

function decode_idx(idx_bytes) {
    return idx_bytes[0] | (idx_bytes[1] << 8) | (idx_bytes[2] << 16);
}

function solve(input, soln_data, soln_bytes) {
    let soln_slice = soln_data.solutions[input];
    let [lower, upper] = [soln_slice[0] * IDX_SIZE, soln_slice[1] * IDX_SIZE];
    let solns = [];
    for (let i = lower;i < upper;i+=IDX_SIZE) {
        solns.push(soln_data.words[decode_idx(soln_bytes.slice(i, i+IDX_SIZE))]);
    }
    return solns;
}

function ui_solve() {
    const start = performance.now();
    const input = document.getElementById("input_letters").value;
    const solns = solve(input, soln_data, soln_bytes);
    const end = performance.now();
    const output = `Solved in ${end - start} milliseconds\n\n${solns.toString()}`;
    document.getElementById("outputs").innerHTML = output;
}

function assemble_input_elements() {
    const input_div = document.getElementById("inputs");
    const input_field = document.createElement("input");
    input_field.type = "text";
    input_field.id = "input_letters";
    input_div.appendChild(input_field);
    const submit_button = document.createElement("button");
    submit_button.textContent = "Solve";
    submit_button.addEventListener("click", ui_solve);
    input_div.appendChild(submit_button);
}

async function page_load() {
    const status = document.getElementById("status");
    status.innerHTML = "Setting up - this may take some time...";
    [soln_data, soln_bytes] = await load_data();
    status.innerHTML = "All set. Lets solve some Blossom!";
    assemble_input_elements();
}
