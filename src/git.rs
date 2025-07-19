use git2::{build::CheckoutBuilder, BranchType, Error, Repository};

pub enum CheckoutType {
    LOCAL,
    REMOTE,
}

pub struct Checkout {
    pub branch_name: String,
    pub checkout_type: CheckoutType,
}

impl Checkout {
    pub fn new(branch_name: String, checkout_type: CheckoutType) -> Self {
        Self {
            branch_name,
            checkout_type,
        }
    }
}

pub struct GitManager {
    repo: Repository,
    pub local_branches: Vec<String>,
    pub remote_branches: Vec<String>,
}

impl GitManager {
    pub fn new() -> GitManager {
        let repo = Repository::open_from_env().unwrap();
        let (l, r) = Self::list_branches(&repo).unwrap();
        Self {
            repo,
            local_branches: l,
            remote_branches: r,
        }
    }

    fn list_branches(repo: &Repository) -> Result<(Vec<String>, Vec<String>), Error> {
        let all_branches = |branch_type| -> Result<Vec<String>, Error> {
            Ok(repo
                .branches(Some(branch_type))?
                .filter_map(Result::ok)
                .filter_map(|(branch, _)| branch.name().ok().flatten().map(|s| s.to_string()))
                .collect::<Vec<_>>())
        };

        Ok((
            all_branches(BranchType::Local)?,
            all_branches(BranchType::Remote)?,
        ))
    }

    pub fn checkout(&self, branch_name: &String) -> Result<Checkout, Error> {
        if self.local_branches.contains(branch_name) {
            return self.checkout_local(branch_name);
        }
        self.checkout_remote(branch_name)
    }

    fn checkout_remote(&self, branch_name: &String) -> Result<Checkout, Error> {
        let local_branch_name = branch_name
            .split_once('/')
            .ok_or_else(|| Error::from_str("Invalid remote branch name"))?
            .1
            .to_string();

        if !self.local_branches.contains(&local_branch_name) {
            return self.checkout_remote_with_tracking(branch_name, &local_branch_name);
        }

        let obj = self
            .repo
            .revparse_single(&format!("refs/heads/{}", local_branch_name))?;
        let local_commit = obj.into_commit().unwrap();

        let remote_ref = format!("refs/remotes/{}", branch_name);
        let remote_reference = self.repo.find_reference(&remote_ref)?;
        let remote_target_commit = remote_reference.peel_to_commit()?;

        if remote_target_commit.id() != local_commit.id() {
            return self.checkout_remote_no_tracking(branch_name);
        }

        self.checkout_local(&local_branch_name)
    }

    fn checkout_local(&self, branch_name: &String) -> Result<Checkout, Error> {
        let mut checkout_opts = CheckoutBuilder::new();
        checkout_opts.safe();

        let obj = self
            .repo
            .revparse_single(&format!("refs/heads/{}", branch_name))?;
        self.repo.checkout_tree(&obj, Some(&mut checkout_opts))?;
        self.repo.set_head(&format!("refs/heads/{}", branch_name))?;

        Ok(Checkout::new(branch_name.clone(), CheckoutType::LOCAL))
    }

    fn checkout_remote_with_tracking(
        &self,
        branch_name: &String,
        local_branch_name: &String,
    ) -> Result<Checkout, Error> {
        let mut checkout_opts = CheckoutBuilder::new();
        checkout_opts.safe();

        let remote_ref = format!("refs/remotes/{}", branch_name);
        let reference = self.repo.find_reference(&remote_ref)?;
        let target_commit = reference.peel_to_commit()?;
        let mut local_branch = self.repo.branch(local_branch_name, &target_commit, false)?;

        local_branch.set_upstream(Some(branch_name))?;
        self.repo
            .checkout_tree(target_commit.as_object(), Some(&mut checkout_opts))?;
        self.repo
            .set_head(&format!("refs/heads/{}", local_branch_name))?;
        Ok(Checkout::new(
            local_branch_name.clone(),
            CheckoutType::LOCAL,
        ))
    }

    fn checkout_remote_no_tracking(&self, branch_name: &String) -> Result<Checkout, Error> {
        let mut checkout_opts = CheckoutBuilder::new();
        checkout_opts.safe();

        let remote_ref = format!("refs/remotes/{}", branch_name);
        let reference = self.repo.find_reference(&remote_ref).unwrap();
        let target_tree = reference.peel_to_tree().unwrap();

        self.repo
            .checkout_tree(target_tree.as_object(), Some(&mut checkout_opts))?;
        self.repo
            .set_head(&format!("refs/remotes/{}", branch_name))?;
        Ok(Checkout::new(branch_name.clone(), CheckoutType::REMOTE))
    }
}
