<script lang="ts">
	import * as d3 from 'd3';

	export let cnt_contained_search_term: number | undefined;
	export let cnt_not_contained_search_term: number | undefined;
	export let width: number;
	export let height: number;
	let created = false;

	$: {
		if (cnt_contained_search_term && cnt_not_contained_search_term && created==false) {
			createPieChart(cnt_contained_search_term, cnt_not_contained_search_term, width, height);
			//created=true;
		}
	}

	async function createPieChart(
		cnt_contained_search_term: number | undefined,
		cnt_not_contained_search_term: number | undefined,
		width: number,
		height: number
	) {
		const margin = 41;
		let data = [cnt_contained_search_term, cnt_not_contained_search_term];
		const radius = Math.min(width, height) / 2 - margin;

		d3.selectAll("svg").remove()
		

		const svg = d3
			.select('#pie-chart')
			.append('svg')
			.attr('width', width)
			.attr('height', height)
			.append('g')
			.attr('transform', `translate(${width / 2},${height / 2})`);

		const color = d3.scaleOrdinal().range(['#00bb00', '#bb0000']);
		const pie = d3.pie().value((d) => d);
		const data_ready = pie(data);
		svg
			.selectAll('pie-as')
			.data(data_ready)
			.join('path')
			.attr(
				'd',
				d3
					.arc()
					.innerRadius(100) // This is the size of the donut hole
					.outerRadius(radius)
			)
			.attr('fill', (d, i) => color(i))
			.attr('stroke', 'black')
			.style('stroke-width', '2px')
			.style('opacity', 0.7);
	}
</script>

<div id="pie-chart" class="pie-chart-container"/>


<style lang=scss>
	.pie-chart-container{
		transition: 100ms;
	}
</style>