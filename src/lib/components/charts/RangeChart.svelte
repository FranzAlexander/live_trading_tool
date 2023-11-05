<script lang="ts">
	import type { RangeBar } from "$lib/range";
	import type { IChartApi, ISeriesApi } from "lightweight-charts";

    export let rangeBarData: RangeBar[];

    let chart: IChartApi;
    let rangeBars: ISeriesApi<'Candlestick'>;

    let chartContainer: HTMLElement;
</script>