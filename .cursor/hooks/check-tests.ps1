# Hook script to check if tests pass before stopping agent
# This runs after agent completes to verify tests still pass

$input = Get-Content $input | ConvertFrom-Json

# Only check if agent completed successfully
if ($input.status -ne "completed") {
    Write-Output (ConvertTo-Json @{})
    exit 0
}

# Run cargo test
$testResult = cargo test 2>&1
$testExitCode = $LASTEXITCODE

if ($testExitCode -eq 0) {
    # Tests pass, allow agent to stop
    Write-Output (ConvertTo-Json @{})
} else {
    # Tests failed, continue agent to fix
    Write-Output (ConvertTo-Json @{
        followup_message = "Tests failed. Please fix the failing tests before completing."
    })
}
