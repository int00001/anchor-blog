use anchor_lang::prelude::*;

declare_id!("CMF1K4xXdEFzz7G3V7y2KW4uBfnT7u2sXn9RBq99qgGq");

// mapping with PDAs
// 1:1 user to blog
// 1:N blog to posts

#[program]
pub mod anchor_blog {
    use super::*;

    pub fn initialize_blog(ctx: Context<InitializeBlog>, blog: Blog) -> Result<()> {
        ctx.accounts.blog_account.set_inner(blog);
        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, post: Post) -> Result<()> {
        if post.title.len() > 20 || post.content.len() > 50 {
            return err!(ErrorCode::InvalidContentOrTitle);
        }

        ctx.accounts.post_account.set_inner(post);
        ctx.accounts.blog_account.post_count += 1;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeBlog<'info> {
    #[account(
        init,
        seeds = [
            b"blog".as_ref(),
            authority.key().as_ref()
        ],
        bump,
        payer = authority,
        space = Blog::LEN
    )]
    pub blog_account: Account<'info, Blog>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(post: Post)]
pub struct CreatePost<'info> {
    #[account(mut, has_one = authority)]
    pub blog_account: Account<'info, Blog>,
    #[account(
        init,
        seeds = [
            b"post".as_ref(),
            blog_account.key().as_ref(),
            post.slug.as_ref(),
        ],
        bump,
        payer = authority,
        space = Post::LEN
    )]
    pub post_account: Account<'info, Post>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Blog {
    pub authority: Pubkey,
    pub bump: u8,
    pub post_count: u8,
}

#[account]
pub struct Post {
    pub author: Pubkey,
    pub slug: String,
    pub title: String,
    pub content: String,
}

impl Blog {
    const LEN: usize = 8 + 32 + 1 + (4 + (10 * 32));
}

impl Post {
    const LEN: usize = 8 + 32 + 32 + (4 + 10) + (4 + 20) + (4 + 50);
}

#[error_code]
pub enum ErrorCode {
    #[msg("invalid content/title")]
    InvalidContentOrTitle,
}
