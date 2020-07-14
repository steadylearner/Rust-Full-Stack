import { divide_by_number_of_participants } from './rust.rs';

const fromRust = divide_by_number_of_participants(10);

const target = document.getElementsByClassName("rust")[0];
target.innerText = fromRust;
