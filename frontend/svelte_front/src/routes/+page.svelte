<script lang="ts">
	import { ArticleStatsProfile } from '$lib/types';
	import { onMount } from 'svelte';
	import PieChart from '$lib/charts/PieChart.svelte';

	const restServices = {
		rs_rest: {
			name: 'Rust: Axum rest service',
			url: 'http://localhost:22550'
		}
	};

	let currentRestService = restServices.rs_rest;
	let currentSite = 'TV2';
	let currentSearchTerm = 'Ukraine';
	let currentArticleProfile: ArticleStatsProfile | null = null;
	let pctMatching = 0

	$: pctMatching = (Math.round((currentArticleProfile?.cntMatches/ currentArticleProfile?.total)*10000))/100


	async function getArticleStats(site: string, searchTerm: string) {
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
		getArticleStats(currentSite, currentSearchTerm);
	});
</script>

<p>Site: {currentSite}</p>
<p>Search Term: {currentSearchTerm}</p>
<p>Total articles: {currentArticleProfile?.total}</p>
<p>Matching search term: {currentArticleProfile?.cntMatches}</p>
<p>Not matching search term: {currentArticleProfile?.cntNonMatches}</p>
<p>PCT matching: {pctMatching}%</p>
<p>Current rest service: {currentRestService.name}</p>

<PieChart
	cnt_contained_search_term={currentArticleProfile?.cntMatches}
	cnt_not_contained_search_term={currentArticleProfile?.cntNonMatches}
	width = 450
	height = 450
/>


