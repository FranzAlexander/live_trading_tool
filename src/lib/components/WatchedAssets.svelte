<script lang="ts">
	export let symbols: string[];
</script>

<div>
	{#each symbols as symbol}
		<p class="text-white">{symbol}</p>
	{/each}
</div>
