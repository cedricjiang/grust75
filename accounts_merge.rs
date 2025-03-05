use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut parent_map = HashMap::new();
        let mut email_name_map = HashMap::new();

        for account in &accounts {
            let name = &account[0];

            for email in &account[1..] {
                parent_map.insert(email, email);
                email_name_map.insert(email, name);
            }
        }

        for account in &accounts {
            let first_email = &account[1];

            for email in &account[2..] {
                let mut involved_emails = HashSet::new();

                let mut email_on_route = first_email;
                loop {
                    involved_emails.insert(email_on_route);
                    if email_on_route == parent_map[email_on_route] {
                        break;
                    }
                    email_on_route = parent_map[email_on_route];
                }

                email_on_route = email;
                while email_on_route != parent_map[email_on_route] {
                    involved_emails.insert(email_on_route);
                    email_on_route = parent_map[email_on_route];
                }
                let parent_email = email_on_route;

                for involved_email in involved_emails {
                    parent_map.insert(involved_email, parent_email);
                }
            }
        }

        let mut reverse_map = HashMap::new();
        let mut root_set = HashSet::new();
        for (email, parent_email) in parent_map {
            if email == parent_email {
                root_set.insert(email);
            } else {
                reverse_map
                    .entry(parent_email)
                    .or_insert(Vec::new())
                    .push(email);
            }
        }

        let mut result_vec = Vec::new();

        for root in root_set {
            let mut vec = Vec::new();
            vec.push(email_name_map[root].clone());
            vec.push(root.clone());

            if let Some(email_queue) = reverse_map.get(root) {
                let mut email_queue = email_queue.clone();

                while let Some(email) = email_queue.pop() {
                    vec.push(email.clone());
                    if let Some(children) = reverse_map.get(email) {
                        for child in children {
                            email_queue.push(child);
                        }
                    }
                }
            }

            (&mut vec[1..]).sort();
            result_vec.push(vec);
        }

        result_vec
    }
}
