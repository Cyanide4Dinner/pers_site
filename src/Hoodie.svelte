<div id="hoodie-container">
	<canvas id = "hoodie-canvas" />
	<div id="greeting-container">
		<span id="greeting-span">> Greetings, I'm <b>Utkarsh Dwivedi. </b></span>
	</div>
	<div id="brief-bio-container">
		<span><b>> ls skills </b></span><span class="cursor-blink">█</span>
		<div id="skill-list-container">
			<div id="skill-list-item"><span>C</span></div>
			<div id="skill-list-item"><span>C++</span></div>
			<div id="skill-list-item"><span>Rust</span></div>
			<div id="skill-list-item"><span>Java</span></div>
			<div id="skill-list-item"><span>Go</span></div>
			<div id="skill-list-item"><span>Python</span></div>
			<div id="skill-list-item"><span>TypeScript</span></div>
		</div>
		<div id="skill-list-container">
			<div id="skill-list-item"><span>Kubernetes</span></div>
			<div id="skill-list-item"><span>Cloud-Computing</span></div>
			<div id="skill-list-item"><span>Microservices</span></div>
			<div id="skill-list-item"><span>Docker</span></div>
			<div id="skill-list-item"><span>AWS</span></div>
		</div>
		<div id="skill-list-container">
			<div id="skill-list-item"><span>Databases</span></div>
			<div id="skill-list-item"><span>Network-Programming</span></div>
			<div id="skill-list-item"><span>Distributed-Systems</span></div>
		</div>
		<div id="skill-list-container">
			<div id="skill-list-item"><span>gRPC</span></div>
			<div id="skill-list-item"><span>REST</span></div>
			<div id="skill-list-item"><span>Spring</span></div>
		</div>
		<div id="skill-list-container">
			<div id="skill-list-item"><span>Bash</span></div>
			<div id="skill-list-item"><span>Unix</span></div>
		</div>
	</div>
</div>

<style>
#hoodie-container {
	position: relative;
	top: 100px;
	left: 100px;
	z-index: 2;
	border: solid green;
	width: 750px;
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
	font-size: 30px;
}

#brief-bio-container {
	position: absolute;
	bottom: 0;
	left: 0;
	width: 700px;
	z-index: 3;
	background-color: #00FF0C;
	font-family: 'Roboto-Mono', monospace;
	margin: 25px;
	padding: 5px;
}

#skill-list-item {
	padding-right: 20px;
}

#skill-list-container{
	display: flex;
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
