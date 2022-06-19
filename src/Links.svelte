<script>
function getMobileWidth() {
	if(window.innerWidth > 600) {
		return "506px"
	} else {
		return "306px";
	}
}
</script>

<div id="links-container"
	class="{window.innerWidth < 1200 ? 'links-container-mobile' : 'links-container-desktop'}"
	style="width: {window.innerWidth < 1200 ? getMobileWidth() : "7vw"}"
>
{#if window.innerWidth <= 1200}
	<div id="links-heading-container">
		<span><b>/links</b></span>
	</div>
	<div id="links-content-container">
		<span class="links-item">/GitHub</span>
		<span class="links-item">/LinkedIn</span>
		<span class="links-item">/Blog</span>
		<span class="links-item">/Resume</span>
	</div>
{/if}
{#if window.innerWidth > 1200}
	<div id="links-heading-container">
		<span><b>/links</b></span>
	</div>
	<div id="links-content-container">
		<span class="links-item">/GitHub</span>
		<br>
		<span class="links-item">/LinkedIn</span>
		<br>
		<span class="links-item">/Blog</span>
		<br>
		<span class="links-item">/Resume</span>
	</div>
{/if}
</div>

<style>
#links-container {
	position: relative;
	display: inline-block;
	z-index: 2;
	border: solid #00FF0C;
	background-color: #000000;
}

.links-container-desktop {
	font-size: 1vw;
	width: 7vw;
	height: 8vw;
}

.links-container-mobile {
	font-size: 0.8rem;
}

#links-heading-container, #links-content-container {
	font-family: 'Roboto-Mono', monospace;
	padding: 2px 5px;
}

#links-heading-container {
	background-color: #00FF0C;
}

#links-content-container {
	background-color: #000000;
	color: #00FF0C;
}

.links-item {
	cursor: pointer;
	text-decoration: underline;
}

.links-item:hover {
	background-color: #00FF0C;
	color: #000000;
}
</style>
