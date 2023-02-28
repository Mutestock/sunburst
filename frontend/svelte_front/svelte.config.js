import adapter from '@sveltejs/adapter-auto';
import preprocessor from "svelte-preprocess";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: preprocessor(),

	kit: {
		adapter: adapter()
	}
};

export default config;
