<script lang="ts">
	import { ArticleStatsProfile } from '$lib/types';
	import { onMount } from 'svelte';

	const restServices = {
		rs_rest: {
			url: 'http://localhost:22550'
		}
	};

	let currentRestService = restServices.rs_rest;
	let currentSite = 'TV2';
	let currentSearchTerm = 'Ukraine';
	let currentArticleProfile: ArticleStatsProfile | null = null;

	$: console.log(currentArticleProfile?.total);

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

<section />
