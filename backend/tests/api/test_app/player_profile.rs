use crate::test_app::base::TestApp;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token_2022::spl_token_2022::solana_program::instruction::Instruction;
use backend_lib::blockchain::solana::helpers::{
    create_transaction, get_account, get_recent_blockhash, send_and_confirm_transaction,
};
use backend_lib::blockchain::solana::pdas::{
    get_player_profile_address, get_player_profile_token_address,
};
use backend_lib::blockchain::solana::player_profile::{
    init_player_profile, init_player_profile_token_account_instruction, init_signer_token_account,
    player_profile_deposit_instruction, player_profile_withdraw_instruction,
};
use backend_lib::errors::error::Error;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use std::sync::Arc;

impl TestApp {
    pub async fn deposit(
        &mut self,
        signer: &Keypair,
        mint: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<()> {
        let mut instructions: Vec<Instruction> = Vec::new();
        let signer_str = signer.to_base58_string();
        let signer = Arc::new(Keypair::from_base58_string(&signer_str));
        let signer_token_account = get_associated_token_address(&signer.pubkey(), mint);
        let (player_profile, _) = get_player_profile_address(&signer.pubkey());
        let (player_token_account, _) = get_player_profile_token_address(&signer.pubkey(), mint);

        let signer_token_account_exists =
            get_account(&self.app_state.client.clone(), &signer_token_account).await?;
        if signer_token_account_exists.is_none() {
            instructions.extend(init_signer_token_account(&signer.clone(), mint)?);
        }
        let player_profile_exists =
            get_account(&self.app_state.client.clone(), &player_profile).await?;
        if player_profile_exists.is_none() {
            instructions.extend(init_player_profile(&signer.clone(), mint)?);
        }
        let player_token_account_exists =
            get_account(&self.app_state.client.clone(), &player_token_account).await?;
        if player_token_account_exists.is_none() {
            instructions.extend(init_player_profile_token_account_instruction(
                &signer.clone(),
                mint,
            )?);
        }

        instructions.extend(player_profile_deposit_instruction(
            &signer.clone(),
            mint,
            amount,
        )?);
        let recent_blockhash = get_recent_blockhash(&self.app_state.client.clone())
            .await
            .map_err(Error::from)?;
        let tx = create_transaction(instructions, &signer.pubkey(), &signer, recent_blockhash)
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        send_and_confirm_transaction(&self.app_state.client.clone(), &tx)
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        Ok(())
    }

    pub async fn withdraw(
        &mut self,
        signer: &Keypair,
        mint: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<()> {
        let context = self.context.lock().await;
        let signer_str = signer.to_base58_string();
        let signer = Arc::new(Keypair::from_base58_string(&signer_str));
        let instructions = player_profile_withdraw_instruction(&signer.clone(), mint, amount)?;
        let recent_blockhash = get_recent_blockhash(&self.app_state.client)
            .await
            .map_err(Error::from)?;
        let tx = create_transaction(
            instructions,
            &context.payer.pubkey(),
            &signer,
            recent_blockhash,
        )
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        send_and_confirm_transaction(&self.app_state.client.clone(), &tx)
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        Ok(())
    }
}
