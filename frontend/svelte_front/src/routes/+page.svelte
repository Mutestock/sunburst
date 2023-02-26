<script lang="ts">
	import { ArticleStatsProfile, RestServiceOptions } from '$lib/types';
	import { onMount } from 'svelte';
	import PieChart from '$lib/charts/PieChart.svelte';

	const restServices: RestServiceOptions[] = [
		new RestServiceOptions('Rust: Axum rest service', 'http://localhost:22550', "Rust"),
		new RestServiceOptions('Python: FastAPI', 'http://localhost:22551', "Python"),
		new RestServiceOptions('C#: .NET Web', 'http://localhost:22552', "C#"),
		new RestServiceOptions('Javascript: Express', 'http://localhost:22553', "Javascript"),
	];
	const sites = ["TV2"]

	let currentRestService: RestServiceOptions = restServices[0];
	let currentSite = 'TV2';
	let currentSearchTerm = 'Ukraine';
	let currentArticleProfile: ArticleStatsProfile | null = null;
	let pctMatching = 0
	let answer = '';

	$: pctMatching = (Math.round((currentArticleProfile?.cntMatches/ currentArticleProfile?.total)*10000))/100
	$: getArticleStats(currentSite, currentSearchTerm, currentRestService);


	async function getArticleStats(site: string, searchTerm: string, restService: RestServiceOptions) {
		let url = `${currentRestService.url}/article/count/site=${site}/search=${searchTerm}`;
		const response = await fetch(url, {
			method: 'GET'
		});
		let resJson = await response.json();
		currentArticleProfile = new ArticleStatsProfile(
			site,
			searchTerm,
			resJson.total,
			resJson.cnt_contained_search_term,
			resJson.cnt_not_contained_search_term
		);
	}

	onMount(async () => {
		getArticleStats(currentSite, currentSearchTerm, currentRestService);
	});
</script>

<p>Site: {currentSite}</p>
<p>Search Term: {currentSearchTerm}</p>
<p>Total articles: {currentArticleProfile?.total}</p>
<p>Matching search term: {currentArticleProfile?.cntMatches}</p>
<p>Not matching search term: {currentArticleProfile?.cntNonMatches}</p>
<p>PCT matching: {pctMatching}%</p>
<p>Current rest service: {currentRestService.name}</p>

<select bind:value={currentRestService} on:change="{() => answer = ''}">
	{#each restServices as restService}
		<option value={restService}>
			{restService.name}
		</option>
	{/each}
</select>



<PieChart
	cnt_contained_search_term={currentArticleProfile?.cntMatches}
	cnt_not_contained_search_term={currentArticleProfile?.cntNonMatches}
	width = 450
	height = 450
/>


