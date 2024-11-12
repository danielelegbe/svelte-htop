<script lang="ts">
	import { systemState } from '$lib/system-state.svelte';
	import CpuBar from '$lib/components/cpu-bar.svelte';

	$effect(() => {
		const connection = systemState.fetchUsage();

		return () => {
			connection.close();
		};
	});
</script>

<main class="flex flex-col items-center justify-center space-y-8 p-4">
	<div class="space-y-4 text-center">
		<h1 class="text-4xl">Memory Usage</h1>
		{#if systemState.memoryUsage && systemState.totalMemory}
			<div>
				<span>Total: {systemState.totalMemory}GB</span>
				<span>Used: {systemState.memoryUsage} GB</span>
			</div>
		{/if}
	</div>

	<h1 class="text-4xl">CPU Usage</h1>

	<div class="flex w-full justify-center gap-2">
		{#if systemState.cpuUsage}
			{#each systemState.cpuUsage as usage, index}
				<CpuBar cpuValue={usage} {index} />
			{/each}
		{/if}
	</div>
</main>
