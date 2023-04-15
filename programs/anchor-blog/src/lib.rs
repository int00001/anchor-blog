use anchor_lang::prelude::*;

declare_id!("CMF1K4xXdEFzz7G3V7y2KW4uBfnT7u2sXn9RBq99qgGq");

// mapping with PDAs
// 1:1 user to blog
// 1:N blog to posts

#[program]
pub mod anchor_blog {
    use super::*;

    // no ix data needed to initialize
    pub fn initialize_blog(ctx: Context<InitializeBlog>) -> Result<()> {
        // set blog account authority and post count
        let new_blog = &mut ctx.accounts.blog_account;
        new_blog.authority = ctx.accounts.authority.key();
        new_blog.post_count = 0;

        msg!(
            "blog created for {} post count {}",
            new_blog.authority,
            new_blog.post_count
        );

        Ok(())
    }

    // ix data is post data
    pub fn create_post(
        ctx: Context<CreatePost>,
        author: Pubkey,
        slug: String,
        title: String,
        content: String,
    ) -> Result<()> {
        if title.len() > 20 || content.len() > 50 {
            return err!(ErrorCode::InvalidContentOrTitle);
        }

        // set post account data
        let new_post = &mut ctx.accounts.post_account;
        new_post.author = author;
        new_post.slug = slug;
        new_post.title = title;
        new_post.content = content;

        // update blog's post count
        ctx.accounts.blog_account.post_count += 1;

        // note: shouldn't post account store blog account it belongs to?
        // ..

        msg!(
            "post created for blog. post count {}",
            ctx.accounts.blog_account.post_count
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeBlog<'info> {
    #[account(
        init_if_needed,
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
