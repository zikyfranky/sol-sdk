#!/bin/bash
set -e 
set -o pipefail

cluster="devnet"
skip=false
key="$(eval echo ~/.config/solana/id.json)"
program_name="skwizz"

# Parse command line arguments
while (( $# > 1 )); do 
  case $1 in
    --cluster)
      cluster="$2"
      ;;
    -c)
      cluster="$2"
      ;;
    --program)
      program_name="$2"
      ;;
    -p)
      program_name="$2"
      ;;
    --skip-checks)
      skip=true
      ;;
    -s)
      skip=true
      ;;
    --key)
      key="$(eval echo $2)"
      ;;
    -k)
      key="$(eval echo $2)"
      ;;
    *)
      break
      ;;
  esac
  shift 2
done

echo $skip

program_deploy_path="target/deploy/$program_name.so"
program_key="target/deploy/$program_name-keypair.json"
program=$(solana-keygen pubkey $program_key)
program_balance=$(solana balance $program -u $cluster)

deployer=$(solana-keygen pubkey $key)
deployer_balance=$(solana balance $deployer -u $cluster)

if [ $skip == false ]; then
  # Run build and tests first.
  echo -e "\nBuilding program ID $program for Solana $cluster.\n"
  yarn build-program
  echo -e "\nBuild finished!\n"
  if [ $cluster == "localhost" ]; then
    echo -e "\Deploying $program for Solana $cluster.\n"
    anchor deploy
    echo -e "\nBuild finished!\n"
  else
  	echo -e "\nRunning tests for Solana $cluster...\n"
  	yarn test || { echo -e "\nTests failed! Please ensure tests are passing before attempting to deploy the program.\n"; exit 1; }
  	echo -e "All checks passed! Building program...\n"
  fi
fi

echo -e "Deployer address $deployer has $deployer_balance\n"
echo -e "Program address $program has $program_balance\n"

# Deploy program.
echo -e "Calling solana program deploy $program_deploy_path -u $cluster -k $key --program-id $program\n"
echo -e "This will take a moment...\n"
solana program deploy $program_deploy_path -u $cluster -k $key --program-id $program --max-len 5000000

echo -e "Program deploy to $cluster finished successfully!\n"
exit 0
