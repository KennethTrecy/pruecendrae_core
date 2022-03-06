#!/usr/bin/env pwsh
param($mode)

if ($mode -eq "remove") {
	& git revert 86a573af
	Write-Output "Removed the license."
}

if ($mode -eq "retain") {
	Write-Output "License will be retained."
}

Remove-Item revert_commits_to.ps1
git add revert_commits_to.ps1
git commit -m "Remove reverse script"
Write-Output "Removed this script."
