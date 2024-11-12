type UsageResponse = {
	cpu_usage: number[];
	memory_usage: number;
	total_memory: number;
};

class SystemState {
	memoryUsage = $state<string | null>(null);
	totalMemory = $state<string | null>(null);
	cpuUsage = $state<number[] | null>(null);

	fetchUsage() {
		const connection = new WebSocket('/api/usage');

		connection.onmessage = (event) => {
			const data = JSON.parse(event.data) as UsageResponse;

			this.memoryUsage = data.memory_usage.toFixed(2);
			this.totalMemory = data.total_memory.toFixed(2);
			this.cpuUsage = data.cpu_usage;
		};

		return connection;
	}
}

export const systemState = new SystemState();
