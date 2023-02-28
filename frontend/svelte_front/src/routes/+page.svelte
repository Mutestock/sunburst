<script lang="ts">
	import { ArticleCount, ArticleStatsProfile, Casing, RestServiceOptions } from '$lib/types';
	import { onMount } from 'svelte';
	import PieChart from '$lib/charts/PieChart.svelte';
	import ProfileTab from '$lib/profile_tab.svelte';

	const restServices: RestServiceOptions[] = [
		new RestServiceOptions('Rust: Axum', 'http://localhost:22550', 'Rust', Casing.SnakeCase),
		new RestServiceOptions('Python: FastAPI', 'http://localhost:22551', 'Python', Casing.SnakeCase),
		new RestServiceOptions('C#: .NET Web', 'http://localhost:22552', 'C#', Casing.CamelCase),
		new RestServiceOptions(
			'Javascript: Express',
			'http://localhost:22553',
			'Javascript',
			Casing.SnakeCase
		)
	];
	const sites = ['TV2', 'DR'];

	let currentRestService: RestServiceOptions = restServices[0];
	let currentSite = 'TV2';
	let currentSearchTerm = 'Ukraine';
	let currentArticleProfile: ArticleStatsProfile | null = null;
	let pctMatching = 0;

	$: if (currentArticleProfile) {
		pctMatching =
			Math.round(
				(currentArticleProfile?.articleCount.cntMatches /
					currentArticleProfile?.articleCount.total) *
					10000
			) / 100;
	}
	$: if (currentSearchTerm) {
		getArticleStats(currentSite, currentSearchTerm, currentRestService);
	}

	async function getArticleStats(
		site: string,
		searchTerm: string,
		restService: RestServiceOptions
	) {
		let url = `${restService.url}/article/count/site=${site}/search=${searchTerm}`;
		const response = await fetch(url, {
			method: 'GET'
		});
		let articleCount = await ArticleCount.fromResponse(response, restService.casing);
		currentArticleProfile = new ArticleStatsProfile(site, searchTerm, articleCount);
	}

	onMount(async () => {
		getArticleStats(currentSite, currentSearchTerm, currentRestService);
	});
</script>

<div class="graphs-page-container">
	<ProfileTab />
	<div class="graphs-page-container-inner">
		<div class="options-tab">
			<select bind:value={currentRestService}>
				{#each restServices as restService}
					<option value={restService}>
						{restService.name}
					</option>
				{/each}
			</select>

			<select bind:value={currentSite}>
				{#each sites as site}
					<option value={site}>
						{site}
					</option>
				{/each}
			</select>
			<input type="text" bind:value={currentSearchTerm} />
		</div>
		<div>
			<p>Site: {currentSite}</p>
			<p>Search Term: {currentSearchTerm}</p>
			<p>Total articles: {currentArticleProfile?.articleCount.total}</p>
			<p>Matching search term: {currentArticleProfile?.articleCount.cntMatches}</p>
			<p>Not matching search term: {currentArticleProfile?.articleCount.cntNonMatches}</p>
			<p>PCT matching: {pctMatching}%</p>
			<p>Current rest service: {currentRestService.name}</p>
			<p>
				Results do get refreshed whenever the rest service is swapped. However, they're configured
				to return exactly the same values
			</p>
			<p>
				That's the point. REST is an architecture style. It has basic constraints which all of the
				services adhere to regardless of language. This is how it's supposed to work.
			</p>

			<PieChart
				cnt_contained_search_term={currentArticleProfile?.articleCount.cntMatches}
				cnt_not_contained_search_term={currentArticleProfile?.articleCount.cntNonMatches}
				width={450}
				height={450}
			/>
		</div>
	</div>
</div>

<style lang="scss">
	.graphs-page-container {
		display: flex;
	}
	.options-tab {
		display: flex;
		flex-direction: row;
		height: 20px;
	}
	.graphs-page-container-inner {
		display: flex;
		flex-direction: column;
		margin: 10px;
	}
</style>
