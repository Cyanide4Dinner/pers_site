<script>
function getBlockSize() {
		var portrait_block_size = 15;
		if (window.innerWidth < 600) {
			portrait_block_size = 6;
		} else if (window.innerWidth >= 600 && window.innerWidth < 1200) {
			portrait_block_size = 10;
		}
		if(window.innerWidth >= 1200 && window.innerWidth < 1900) {
			portrait_block_size = 10;
		} else if(window.innerWidth >= 1900 && window.innerWidth < 2500) {
			portrait_block_size = 15;
		} else if (window.innerWidth >= 2500 && window.innerWidth < 3800) {
			portrait_block_size = 20;
		} else if (window.innerWidth >= 3800) {
			portrait_block_size = 30;
		}
		return portrait_block_size;
}
</script>

<div id="hoodie-container" style="width: {getBlockSize() * 50}px; height: {getBlockSize() * 52}px">
	<canvas id = "hoodie-canvas" />
	<div id="greeting-container">
		<span id="greeting-span" style="font-size: {getBlockSize() * 2}px">> Greetings, I'm <b>Utkarsh Dwivedi. </b></span>
	</div>
	<div id="brief-bio-container" 
		style="width: {getBlockSize() * 46}px; margin: {getBlockSize()}px; padding: {getBlockSize()}px"
		class="{window.innerWidth < 1200 ? 'brief-bio-container-mobile' : 'brief-bio-container-desktop'}"
		>
		<span><b>> ls skills </b></span><span class="cursor-blink">█</span>
		<div id="skill-list-container" >
			<div style="padding-right: {getBlockSize()}px"><span>C</span></div>
			<div style="padding-right: {getBlockSize()}px"><span>C++</span></div>
			<div style="padding-right: {getBlockSize()}px"><span>Rust</span></div>
			<div style="padding-right: {getBlockSize()}px"><span>Java</span></div>
			<div style="padding-right: {getBlockSize()}px"><span>Python</span></div>
		</div>
		<div id="skill-list-container">
			<div style="padding-right: {getBlockSize()}px"><span>Kubernetes</span></div>
			<div style="padding-right: {getBlockSize()}px">Microservices</div>
			<div style="padding-right: {getBlockSize()}px"><span>Docker</span></div>
			<div style="padding-right: {getBlockSize()}px"><span>Cloud-Computing</span></div>
			<div style="padding-right: {getBlockSize()}px"><span>Databases</span></div>
			<div style="padding-right: {getBlockSize()}px"><span>Docker</span></div>
			<div style="padding-right: {getBlockSize()}px"><span>Cloud-Computing</span></div>
			<div style="padding-right: {getBlockSize()}px"><span>Databases</span></div>
		</div>
	</div>
</div>

<style>
#hoodie-container {
	position: relative;
	z-index: 2;
	border: solid #00FF0C;
}

#greeting-container {
	position: absolute;
	color: rgb(0, 255, 65);
	font-family: 'Roboto-Mono', monospace;
	top: 0;
	left: 0;
	z-index: 3;
	text-align: center;
	width: 100%;
	padding: 4px;
}

#brief-bio-container {
	position: absolute;
	bottom: 0;
	left: 0;
	z-index: 3;
	background-color: #00FF0C;
	font-family: 'Roboto-Mono', monospace;
}

.brief-bio-container-desktop {
	font-size: 1vw;
}

.brief-bio-container-mobile {
	font-size: 0.8rem;

}

#skill-list-container{
	display: flex;
	flex-wrap: wrap;
}

.cursor-blink {
	animation: blink-animation 1s steps(5, start) infinite;
}
@keyframes blink-animation {
	0% {
		visibility: visible;
	}
	25% {
		visibility: hidden;
	}

	75% {
		visibility: hidden;
	}

	100% {
		visibility: visible;
	}
}
</style>
