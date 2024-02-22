const { spawn, exec } = require('child_process');

// Function to check if the service is ready using netcat
function checkPortOpen(port, callback) {
  exec(`nc -z 127.0.0.1 ${port}`, (error, stdout, stderr) => {
    if (error) {
      console.log(`Port ${port} is not open yet.`);
      return callback(false); // Service is not ready
    }
    console.log(`Port ${port} is open.`);
    callback(true); // Service is ready
  });
}

// Function to wait for the service to be ready
function waitForPort(port, onSuccess, maxAttempts = 20, interval = 3000, attempts = 0) {
  checkPortOpen(port, (isReady) => {
    if (isReady) {
      console.log('Service is ready, proceeding with the main task...');
      onSuccess();
    } else if (attempts < maxAttempts) {
      console.log('Waiting for the service to be ready...');
      setTimeout(() => waitForPort(port, onSuccess, maxAttempts, interval, attempts + 1), interval);
    } else {
      console.log('Service did not become ready in time, aborting...');
      process.exit(1);
    }
  });
}

// Extract and parse command-line arguments
const args = process.argv.slice(2);
const scriptPortFlagIndex = args.indexOf('--'); // Find the script's '--' separator for port
const port = parseInt(args[scriptPortFlagIndex + 1], 10); // The port to wait on before starting the main task
const bgTaskArgs = args.slice(0, scriptPortFlagIndex);
const mainTaskFlagIndex = args.indexOf('--', scriptPortFlagIndex + 2);
const mainTaskCmd = args.slice(mainTaskFlagIndex + 1);

// Start the background task
console.log(`Starting background task: ${bgTaskArgs.join(' ')}`);
const bgTask = spawn(bgTaskArgs[0], bgTaskArgs.slice(1), { stdio: 'inherit' });

// Ensure the background task is killed on exit and exit with the main task status
function cleanup(exitCode = 0) {
  console.log('Killing the background task...');
  bgTask.kill();
  process.exit(exitCode); // Exit with the provided exit code
}

process.on('SIGINT', () => cleanup());
process.on('SIGTERM', () => cleanup());

// Wait for the specified port to open before starting the main task
waitForPort(port, () => {
  console.log(`Starting main task: ${mainTaskCmd.join(' ')}`);
  const mainTaskProcess = spawn(mainTaskCmd[0], mainTaskCmd.slice(1), { stdio: 'inherit' });

  mainTaskProcess.on('close', (code) => {
    console.log(`Main task exited with code ${code}`);
    cleanup(code); // Ensure background task is killed and exit with the main task's exit code
  });
});
