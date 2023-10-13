const { invoke } = window.__TAURI__.tauri;

function sleep(dur) {
  return new Promise(resolve => setTimeout(resolve, dur));
}

const canvas = document.getElementById("page");
const mainText = document.getElementById("readyText");
const button = document.getElementById("mainButton");

var is_pressed = false;
var counter = 0;
var startTime;

button.setAttribute("disabled", true);

async function game_over() {
	let overText = document.createElement("span");
	overText.innerText = "Вы не прошли испытание. Возвращайтесь просматривать нудные анимации";
	overText.classList = "main-text-fade-unfade";
	document.body.appendChild(overText);
	await sleep(3800);
	location.reload();
}

button.addEventListener("click", async function() {
	if (counter != 1) {
		counter++;
		var randomX = Math.floor(Math.random() * ((document.body.offsetWidth - 100) - 101)) + 100;
		var randomY = Math.floor(Math.random() * ((document.body.offsetHeight - 100) - 101)) + 100;
		button.style.top = randomY + "px";
		button.style.left = randomX + "px";
		var endTime = Date.now();
		if (((endTime - startTime) / 1000) >= 1.2) {
			button.remove();
			await game_over();
		} else {
			startTime = Date.now();
		}
	} else {
		button.remove();
		let good = document.createElement("span");
		good.textContent = "Теперь можно узнать кто я :)";
		good.classList = "main-text-fade-unfade";
		document.body.appendChild(good);
		await sleep(4000);
		good.remove();
		let mainPage = document.createElement("div");
		let image = document.createElement("img");
		let headAcc = document.createElement("div");
		let name = document.createElement("b");
		let bodyAcc = document.createElement("div");
		let infoGit = document.createElement("a");
		let infoTg = document.createElement("span");
		let hr = document.createElement("hr");
		let br = document.createElement("br");
		let license = document.createElement("span");
		mainPage.classList = "main-page";
		document.body.appendChild(mainPage);
		headAcc.classList = "head-account";
		mainPage.appendChild(headAcc);
		mainPage.appendChild(hr);
		image.src = "./assets/88966131.png";
		image.alt = "puffy_with_eyes";
		image.classList = "ava";
		headAcc.appendChild(image);
		name.textContent = "София Изюмская (РУДН)";
		name.classList = "head-text";
		headAcc.appendChild(name);
		bodyAcc.classList = "body-account";
		mainPage.appendChild(bodyAcc);
		infoGit.textContent = "GitHub"
		infoGit.href = "https://github.com/PuffyWithEyes";
		infoTg.textContent = "Telegram: @puffy22";
		infoTg.classList = "tg";
		bodyAcc.appendChild(infoGit);
		bodyAcc.appendChild(br);
		bodyAcc.appendChild(infoTg);
		license.textContent = "GNU 3.0 Licence";
		license.classList = "license";
		bodyAcc.appendChild(license);
	}
});


document.body.addEventListener("click", async function(event) {
	if (!is_pressed) {
		is_pressed = true;
		mainText.classList = "main-text-unfade";
		await sleep(2000);
		mainText.innerText = "Тогда испытание начинается";
		mainText.classList = "main-text-fade-unfade";
		await sleep(4000);
		mainText.remove();
		await sleep(500);
		
		button.removeAttribute("disabled");
		startTime = Date.now();
		var randomX = Math.floor(Math.random() * document.body.offsetWidth - 125);
		var randomY = Math.floor(Math.random() * document.body.offsetHeight - 125);
		button.style.top = randomY + "px";
		button.style.left = randomX + "px";
		button.classList = "round";
	}
});

