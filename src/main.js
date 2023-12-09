const { invoke } = window.__TAURI__.tauri;


let filePath;
let dirPath;
let wordLen;
let lengthList = [];
let dataList = [];


const ctx = document.getElementById('huffmanChart').getContext('2d');
async function drawChart() {
	let compressDict = [];
	let compress = [];
	let bEntropy = [];
	let shenEntropy = [];
	let counter = 0;
	
	for (let item of dataList) {
		compressDict[counter] = item.compression_dict;
		compress[counter] = item.compression;
		bEntropy[counter] = item.b_entropy;
		shenEntropy[counter] = item.shen_entropy;
		counter++;
	}
	
    let myChart = new Chart(ctx, {
	    type: 'line',
	    data: {
		    labels: lengthList,
		    datasets: [{
			    label: 'Коэф. сжатия со словарем',
			    data: compressDict,
			    backgroundColor: 'rgba(255, 99, 132, 0.2)',
			    borderColor: 'rgba(255, 99, 132, 1)',
			    borderWidth: 1
		    }, {
                label: "Коэф. сжатия",
                data: compress,
                backgroundColor: "rgba(0, 252, 201, 0.2)",
                borderColor: "rgba(0, 252, 201, 1)",
                borderWidth: 1
            }, {
                label: "B энтропия",
                data: bEntropy,
                backgroundColor: "rgba(252, 0, 235, 0.2)",
                borderColor: "rgba(252, 0, 235, 1)",
                borderWidth: 1
            }, {
                label: "Шенноновская энтропия",
                data: shenEntropy,
                backgroundColor: "rgba(252, 243, 0, 0.2)",
                borderColor: "rgba(252, 243, 0, 1)",
                borderWidth: 1
            }]
	    },
	    options: {
		    scales: {
			    y: {
				    beginAtZero: true
			    }
		    }
	    }
    });

	return myChart;
}


async function choose_file() {
    filePath.textContent = await invoke("choose_file");
}


window.addEventListener("DOMContentLoaded", () => {
    filePath = document.querySelector("#file");
    document.querySelector("#button-file").addEventListener("submit", (e) => {
        e.preventDefault();
        choose_file();
    });
});


async function choose_dir() {
    dirPath.textContent = await invoke("choose_dir");
}


window.addEventListener("DOMContentLoaded", () => {
    dirPath = document.querySelector("#dir");
    document.querySelector("#button-dir").addEventListener("submit", (e) => {
        e.preventDefault();
        choose_dir();
    });
});


async function calc() {
	ctx.clearRect(0, 0, ctx.width, ctx.height);
    if (filePath.textContent == "" || dirPath.textContent == "") {
        return;
    }

    wordLen = document.getElementById("word-len");

    var length = parseInt(wordLen.value);

	if (length <= 0) {
		return;
	}

	for (let i = 1; i <= length; i++) {
		lengthList[i - 1] = i;
		
		dataList[i - 1] = await invoke("calc", { file_path: filePath.textContent,
												 dir_path: dirPath.textContent,
												 length: i,
												 full_length: length
											   });
	}

	drawChart();
}


window.addEventListener("DOMContentLoaded", () => {
    document.querySelector("#button-calc").addEventListener("submit", (e) => {
        e.preventDefault();
        calc();
    });
});

