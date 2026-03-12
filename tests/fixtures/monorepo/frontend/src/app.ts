export class App {
	private running = false;

	run(): void {
		this.running = true;
		console.log("app started");
	}

	stop(): void {
		this.running = false;
	}

	isRunning(): boolean {
		return this.running;
	}
}
