use async_std::process::Command;

pub async fn create_new_project(project_name: &String) -> anyhow::Result<()> {
  /*
  Do equivalent of:
  npm init svelte $project_name
  pnpm install
  pnpm i @svelteuidev/core @svelteuidev/composables
  updating __layout
  ... or maybe just fetch a prepared template
   */

  let mut child = Command::new("npm")
            .args(&["init", "svelte", project_name])
            .spawn()?;
  let _ = child.status().await?;

  let mut child = Command::new("npm")
            .args(&["install", "@svelteuidev/core", "@svelteuidev/composables"])
            .current_dir(&project_name)
            .spawn()?;
  let _ = child.status().await?;

  Ok(())
}