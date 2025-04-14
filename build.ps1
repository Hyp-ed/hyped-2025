echo --% >/dev/null;: ' | out-null
<#'
echo "Building main workspace"
if cargo build --quiet; then 
    echo "Successfully built main workspace"
else
    echo "Failed to build main workspace"
    exit 1
fi
cd boards || exit 1
for dir_name in *; do
    if [ -d "$dir_name" ]; then
        cd "$dir_name"
        if cargo build --quiet; then 
            echo "Successfully built $dir_name"
        else
            echo "Failed to build $dir_name"
        fi 
        cd ..
    else
        echo "Skipping $dir_name as it is not a directory"
    fi
done
exit
exit #>


Write-Output "Building project root"

cargo build --quiet 
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build project root"
    exit $LASTEXITCODE
}
Write-Output "Project root built successfully"
Write-Output "Building boards"

foreach ($board in Get-ChildItem -Path .\boards -Directory) {
    Write-Output "Building board $($board.Name)"
    Set-Location $board.FullName
    cargo build --quiet 
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to build board $($board.Name)"
        exit $LASTEXITCODE
    }
    Write-Output "Board $($board.Name) built successfully"
    Set-Location ../..
}