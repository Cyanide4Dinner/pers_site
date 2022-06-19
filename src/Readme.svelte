<script>
function getMobileWidth() {
	if(window.innerWidth > 600) {
		return "506px"
	} else {
		return "306px";
	}
}
</script>

<div id="readme-container"
	class="{window.innerWidth < 1200 ? 'readme-container-mobile' : 'readme-container-desktop'}"
	style="width: {window.innerWidth < 1200 ? getMobileWidth() : "40vw"}"
>
	<div id="readme-heading-container">
		<span><b>README.md</b></span>
	</div>
	<div id="readme-content-container">
		<span>
			Computer Science major passionate about diverse areas of the field. 
			Skilled in distributed and cloud architectures, databases and developing performant solutions for backend. Also interested in low-level programming using C and Rust. Graduating in July 2022.
			Computer Science major passionate about diverse areas of the field. 
			Skilled in distributed and cloud architectures, databases and developing performant solutions for backend. Also interested in low-level programming using C and Rust. Graduating in July 2022.
			Computer Science major passionate about diverse areas of the field. 
			Skilled in distributed and cloud architectures, databases and developing performant solutions for backend. Also interested in low-level programming using C and Rust. Graduating in July 2022.
		</span>
	</div>
</div>

<style>
#readme-container {
	position: relative;
	z-index: 2;
	background-color: #000000;
}

.readme-container-desktop {
	width: 40vw;
	font-size: 1vw;
	border: medium solid #00FF0C;
}

.readme-container-mobile {
	font-size: 0.8rem;
	border: solid #00FF0C;
}

#readme-heading-container, #readme-content-container {
	font-family: 'Roboto-Mono', monospace;
	padding: 2px 5px;
}

#readme-heading-container {
	background-color: #00FF0C;
}

#readme-content-container {
	color: #00FF0C;
}
</style>
