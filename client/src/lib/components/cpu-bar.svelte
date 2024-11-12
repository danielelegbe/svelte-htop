<script lang="ts">
	type Props = {
		cpuValue: number;
		index: number;
	};

	const { cpuValue, index }: Props = $props();
</script>

<div class="flex flex-col items-center justify-center space-y-2">
	<div class="overflow-y-none relative h-32 w-16 rounded rounded-b-none bg-gray-200 text-center">
		<div
			class="absolute bottom-0 left-0 right-0 h-full bg-blue-300 transition-all duration-300"
			style="height: {cpuValue.toFixed(2)}%;"
		></div>
		<span class="text-xs text-black">{cpuValue.toFixed(2)}%</span>
	</div>
	<span class="bottom-5 text-white">{index}</span>
</div>
