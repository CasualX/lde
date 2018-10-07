/*
Run this script in scratchpad on http://ref.x86asm.net/geek.html
*/

var tables = Array.from(document.querySelectorAll(".ref_table.notpublic"));

function join(head, tr) {
	let tds = tr.querySelectorAll("td");
	let result = {};
	let j = 0;
	for (let i = 0; i < tds.length; ++i) {
		let td = tds[i];
		result[head[j]] = td.textContent.trim();
		j += td.colSpan || 1;
	}
	return result;
}

function csv(head, data) {
	let sb = [head.join("\x1F")];
	for (let i = 0; i < data.length; ++i) {
		sb.push(head.map(x => data[i][x]).join("\x1F"));
	}
	return sb.join("\n");
}

function dump(table) {
	// Get the headers as a nice array
	let head = Array.from(table.querySelectorAll("thead>tr>th")).map(th => th.textContent.trim());
	let data = [];
	let trs = table.querySelectorAll("tbody>tr:first-child");
	for (let i = 0; i < trs.length; ++i) {
		data.push(join(head, trs[i]));
	}
	return csv(head, data);
}

dump(tables[0])
//dump(tables[1])
