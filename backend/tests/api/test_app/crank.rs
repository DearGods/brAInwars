use crate::test_app::base::TestApp;
use anchor_lang::solana_program::instruction::Instruction;
use backend_lib::blockchain::solana::crank::{init_crank_instruction, set_crank_instruction};
use backend_lib::blockchain::solana::helpers::{
    create_transaction, get_recent_blockhash, send_and_confirm_transaction,
};
use backend_lib::errors::error::Error;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use std::sync::Arc;

impl TestApp {
    pub async fn set_crank(
        &mut self,
        signer: &Keypair,
        crank_signer: &Pubkey,
        fee: u64,
    ) -> anyhow::Result<()> {
        let signer_str = signer.to_base58_string();
        let signer = Arc::new(Keypair::from_base58_string(&signer_str));
        let mut instructions: Vec<Instruction> = Vec::new();
        instructions.extend(init_crank_instruction(&signer).await?);
        instructions.extend(set_crank_instruction(signer.clone(), crank_signer, fee).await?);
        let recent_blockhash = get_recent_blockhash(&self.app_state.client)
            .await
            .map_err(Error::from)?;
        let tx = create_transaction(instructions, &signer.pubkey(), &signer, recent_blockhash)
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        send_and_confirm_transaction(&self.app_state.client.clone(), &tx)
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        Ok(())
    }
}
