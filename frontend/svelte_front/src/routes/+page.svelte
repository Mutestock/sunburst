<script lang="ts">
	import { ArticleCount, ArticleStatsProfile, Casing, RestServiceOptions } from '$lib/types';
	import { onMount } from 'svelte';
	import PieChart from '$lib/charts/PieChart.svelte';

	const restServices: RestServiceOptions[] = [
		new RestServiceOptions('Rust: Axum rest service', 'http://localhost:22550', "Rust", Casing.SnakeCase),
		new RestServiceOptions('Python: FastAPI', 'http://localhost:22551', "Python", Casing.SnakeCase),
		new RestServiceOptions('C#: .NET Web', 'http://localhost:22552', "C#", Casing.CamelCase),
		new RestServiceOptions('Javascript: Express', 'http://localhost:22553', "Javascript", Casing.SnakeCase),
	];
	const sites = ["TV2"]

	let currentRestService: RestServiceOptions = restServices[0];
	let currentSite = 'TV2';
	let currentSearchTerm = 'Ukraine';
	let currentArticleProfile: ArticleStatsProfile | null = null;
	let pctMatching = 0

	$: pctMatching = (Math.round((currentArticleProfile?.articleCount.cntMatches/ currentArticleProfile?.articleCount.total)*10000))/100
	$: getArticleStats(currentSite, currentSearchTerm, currentRestService);


	async function getArticleStats(site: string, searchTerm: string, restService: RestServiceOptions) {
		let url = `${restService.url}/article/count/site=${site}/search=${searchTerm}`;
		const response = await fetch(url, {
			method: 'GET'
		});
		let articleCount = await ArticleCount.fromResponse(response, restService.casing);
		currentArticleProfile = new ArticleStatsProfile(
			site,
			searchTerm,
			articleCount
		);
	}

	onMount(async () => {
		getArticleStats(currentSite, currentSearchTerm, currentRestService);
	});
</script>

<p>Site: {currentSite}</p>
<p>Search Term: {currentSearchTerm}</p>
<p>Total articles: {currentArticleProfile?.articleCount.total}</p>
<p>Matching search term: {currentArticleProfile?.articleCount.cntMatches}</p>
<p>Not matching search term: {currentArticleProfile?.articleCount.cntNonMatches}</p>
<p>PCT matching: {pctMatching}%</p>
<p>Current rest service: {currentRestService.name}</p>

<select bind:value={currentRestService}>
	{#each restServices as restService}
		<option value={restService}>
			{restService.name}
		</option>
	{/each}
</select>



<PieChart
	cnt_contained_search_term={currentArticleProfile?.articleCount.cntMatches}
	cnt_not_contained_search_term={currentArticleProfile?.articleCount.cntNonMatches}
	width = 450
	height = 450
/>


