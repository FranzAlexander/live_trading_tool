<script lang="ts">
	import type { IChartApi } from 'lightweight-charts';

	let macdChart: IChartApi;

	let macdContainer: HTMLElement;
</script>

<div bind:this={macdContainer} />
