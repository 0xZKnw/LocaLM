#![allow(non_snake_case)]

use crate::app::AppState;
use dioxus::prelude::*;

pub fn HelpView() -> Element {
    let app_state = use_context::<AppState>();
    let is_en = app_state.settings.read().language == "en";

    rsx! {
        div {
            class: "flex-1 overflow-y-auto p-6 custom-scrollbar",
            style: "max-width: 800px; margin: 0 auto;",

            // Title
            h1 {
                class: "text-2xl font-bold mb-8",
                style: "color: var(--text-primary);",
                if is_en { "Help & Tutorial" } else { "Aide et Tutoriel" }
            }

            // Getting Started Section
            HelpSection {
                is_en: is_en,
                title_en: "Getting Started",
                title_fr: "Comment commencer",
                icon: "M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253",
                content_en: r#"<p class="mb-4">1. <strong>Load a model</strong>: Click the model selector in the header to choose a .gguf model. If you don't have any models, you can download them from HuggingFace using the sidebar.</p>
<p class="mb-4">2. <strong>Start chatting</strong>: Once a model is loaded, type a message in the chat input and press Enter or click the send button.</p>
<p>3. <strong>Ask for help</strong>: The AI can read files, run commands, search the web, and more. Just ask!</p>"#,
                content_fr: r#"<p class="mb-4">1. <strong>Charger un modele</strong>: Cliquez sur le selecteur de modele dans l'en-tete pour choisir un fichier .gguf. Si vous n'avez pas de modeles, vous pouvez les telecharger depuis HuggingFace via la barre laterale.</p>
<p class="mb-4">2. <strong>Commencer a discuter</strong>: Une fois un modele charge, tapez un message dans la zone de saisie et appuyez sur Entree ou cliquez sur le bouton d'envoi.</p>
<p>3. <strong>Demander de l'aide</strong>: L'IA peut lire des fichiers, executer des commandes, rechercher sur le web, et plus encore. Il suffit de demander!</p>"#
            }

            // Agent Tools Section
            HelpSection {
                is_en: is_en,
                title_en: "Agent Tools",
                title_fr: "Outils de l'agent",
                icon: "M11 5H6a2 2 0 0 0-2 2v11a2 2 0 0 0 2 2h11a2 2 0 0 0 2-2v-5m-1.414-9.414a2 2 0 1 1 2.828 2.828L11.828 15H9v-2.828l8.586-8.586z",
                content_en: r#"<p class="mb-4">ClawRS includes <strong>30+ built-in tools</strong> organized by category:</p>
<ul class="list-disc pl-6 mb-4 space-y-2">
<li><strong>File Operations</strong>: Read, write, edit, search, and manage files and directories</li>
<li><strong>Shell</strong>: Execute bash/PowerShell commands on your system</li>
<li><strong>Git</strong>: Run git operations (status, diff, log, commit, branch, stash)</li>
<li><strong>Web Search</strong>: Search the web and code repositories using Exa AI</li>
<li><strong>Web Fetch</strong>: Download and extract content from URLs</li>
<li><strong>Dev Tools</strong>: Diff, find-replace, patch, and code analysis</li>
<li><strong>System</strong>: Process list, environment variables, system info</li>
</ul>
<p>The AI will automatically suggest which tools to use based on your request.</p>"#,
                content_fr: r#"<p class="mb-4">ClawRS inclut <strong>plus de 30 outils integres</strong> organises par categorie:</p>
<ul class="list-disc pl-6 mb-4 space-y-2">
<li><strong>Operations fichiers</strong>: Lire, ecrire, modifier, rechercher et gerer des fichiers et repertoires</li>
<li><strong>Shell</strong>: Executer des commandes bash/PowerShell sur votre systeme</li>
<li><strong>Git</strong>: Executer des operations git (status, diff, log, commit, branch, stash)</li>
<li><strong>Recherche web</strong>: Rechercher sur le web et les depots de code avec Exa AI</li>
<li><strong>Extraction web</strong>: Telecharger et extraire le contenu des URLs</li>
<li><strong>Outils dev</strong>: Diff, recherche-remplacement, patch et analyse de code</li>
<li><strong>Systeme</strong>: Liste des processus, variables d'environnement, infos systeme</li>
</ul>
<L'IA suggere automatiquement quels outils utiliser en fonction de votre demande.</p>"#
            }

            // Permissions Section
            HelpSection {
                is_en: is_en,
                title_en: "Permissions",
                title_fr: "Permissions",
                icon: "M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z",
                content_en: r#"<p class="mb-4">ClawRS has a <strong>6-level permission system</strong> to control tool access:</p>
<ul class="list-decimal pl-6 mb-4 space-y-2">
<li><strong>ReadOnly</strong>: File read, grep, glob only</li>
<li><strong>Filesystem</strong>: Read + write files</li>
<li><strong>Execute</strong>: Filesystem + shell commands</li>
<li><strong>Git</strong>: Full git access</li>
<li><strong>Network</strong>: Web search and downloads</li>
<li><strong>Admin</strong>: All capabilities</li>
</ul>
<p class="mb-4"><strong>Permission modes</strong>:</p>
<ul class="list-disc pl-6 space-y-2">
<li><strong>Manual approval</strong> (default): Each tool call shows a dialog for you to approve</li>
<li><strong>Allowlist</strong>: Pre-approve specific tools in Settings > Tools</li>
<li><strong>Auto-approve</strong>: Skip all dialogs (use with caution!)</li>
</ul>"#,
                content_fr: r#"<p class="mb-4">ClawRS dispose d'un <strong>systeme de permissions a 6 niveaux</strong> pour controler l'acces aux outils:</p>
<ul class="list-decimal pl-6 mb-4 space-y-2">
<li><strong>ReadOnly</strong>: Lecture de fichiers, grep, glob uniquement</li>
<li><strong>Filesystem</strong>: Lecture + ecriture de fichiers</li>
<li><strong>Execute</strong>: Fichiers + commandes shell</li>
<li><strong>Git</strong>: Acces complet git</li>
<li><strong>Network</strong>: Recherche web et telechargements</li>
<li><strong>Admin</strong>: Toutes les capacites</li>
</ul>
<p class="mb-4"><strong>Modes de permissions</strong>:</p>
<ul class="list-disc pl-6 space-y-2">
<li><strong>Approbation manuelle</strong> (defaut): Chaque appel d'outil affiche une dialogue pour validation</li>
<li><strong>Liste blanche</strong>: Pre-approuver des outils specifiques dans Parametres > Outils</li>
<li><strong>Auto-approuver</strong>: Sauter toutes les dialogues (a utiliser avec precaution!)</li>
</ul>"#
            }

            // Important Limitations Section
            HelpSection {
                is_en: is_en,
                title_en: "Important Limitations",
                title_fr: "Limitations importantes",
                icon: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z",
                content_en: r#"<p class="mb-4">ClawRS runs <strong>entirely offline</strong> using local models. Important constraints:</p>
<ul class="list-disc pl-6 mb-4 space-y-2">
<li><strong>VRAM/RAM</strong>: Models require 4-16GB. Use quantized models (Q4, Q5, Q8) for best results.</li>
<li><strong>Context window</strong>: Limited to 4K-32K tokens. ClawRS automatically adjusts based on your VRAM.</li>
<li><strong>Model quality</strong>: Local models have less knowledge than cloud models (GPT-5, Gemini 3, Claude Opus 4.6).</li>
<li><strong>Reasoning</strong>: Complex multi-step reasoning may be less reliable.</li>
<li><strong>Speed</strong>: Depends on your hardware. GPU acceleration recommended.</li>
</ul>
<p>Best for: coding help, file operations, local tasks. Not ideal for deep research.</p>"#,
                content_fr: r#"<p class="mb-4">ClawRS fonctionne <strong>entierement hors ligne</strong> avec des modeles locaux. Contraintes importantes:</p>
<ul class="list-disc pl-6 mb-4 space-y-2">
<li><strong>VRAM/RAM</strong>: Les modeles necessitent 4-16GB. Utilisez des modeles quantifies (Q4, Q5, Q8) pour de meilleurs resultats.</li>
<li><strong>Fenetre de contexte</strong>: Limitee a 4K-32K tokens. ClawRS ajuste automatiquement selon votre VRAM.</li>
<li><strong>Qualite du modele</strong>: Les modeles locaux ont moins de connaissances que les modeles cloud (GPT-5, Gemini 3, Claude Opus 4.6).</li>
<li><strong>Reasonnement</strong>: Le raisonnement complexe en plusieurs etapes peut etre moins fiable.</li>
<li><strong>Vitesse</strong>: Depend de votre materiel. Acceleration GPU recommandee.</li>
</ul>
<p>Ideal pour: aide au code, operations sur fichiers, taches locales. Pas ideal pour la recherche approfondie.</p>"#
            }

            // Tips Section
            HelpSection {
                is_en: is_en,
                title_en: "Tips",
                title_fr: "Conseils",
                icon: "M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z",
                content_en: r#"<ul class="list-disc pl-6 space-y-3">
<li>Use <strong>quantized models</strong> (Q4_K_M, Q5_K_S) for best speed/quality ratio</li>
<li>Ensure <strong>sufficient VRAM</strong> before loading large models</li>
<li>Keep conversations <strong>focused</strong> to avoid hitting context limits</li>
<li>Use <strong>clear, specific requests</strong> for better results</li>
<li>Check the <strong>Settings</strong> panel to customize inference parameters (temperature, top-p, etc.)</li>
<li>Enable <strong>GPU acceleration</strong> in Hardware settings for faster inference</li>
<li>Pre-approve frequent tools in the <strong>allowlist</strong> to speed up workflows</li>
<li>For complex reasoning, consider using GPT-5, Gemini 3 or Claude Opus 4.6 and use ClawRS for execution</li>
</ul>"#,
                content_fr: r#"<ul class="list-disc pl-6 space-y-3">
<li>Utilisez des <strong>modeles quantifies</strong> (Q4_K_M, Q5_K_S) pour le meilleur ratio vitesse/qualite</li>
<li>Assurez-vous d'avoir <strong>sufficient VRAM</strong> avant de charger de grands modeles</li>
<li>Gardez les conversations <strong>concentrees</strong> pour eviter d'atteindre les limites de contexte</li>
<li>Faites des <strong>requetes claires et specifiques</strong> pour de meilleurs resultats</li>
<li>Consultez le panneau <strong>Parametres</strong> pour personnalier les parametres d'inference (temperature, top-p, etc.)</li>
<li>Activez <strong>l'acceleration GPU</strong> dans les parametres Materiel pour une inference plus rapide</li>
<li>Pre-approuvez les outils frequents dans la <strong>liste blanche</strong> pour accelerer les flux de travail</li>
<li>Pour un raisonnement complexe, utilisez GPT-5, Gemini 3 ou Claude Opus 4.6 et servez-vous de ClawRS pour l'execution</li>
</ul>"#
            }

            // Footer spacing
            div { class: "h-8" }
        }
    }
}

#[component]
fn HelpSection(
    is_en: bool,
    title_en: &'static str,
    title_fr: &'static str,
    icon: &'static str,
    content_en: &'static str,
    content_fr: &'static str,
) -> Element {
    let title = if is_en { title_en } else { title_fr };
    let content = if is_en { content_en } else { content_fr };

    rsx! {
        div {
            class: "glass rounded-2xl p-6 mb-6",
            style: "border: 1px solid rgba(242,237,231,0.08);",

            // Section header
            div {
                class: "flex items-center gap-3 mb-4",
                div {
                    class: "flex-shrink-0 w-8 h-8 rounded-lg",
                    style: "background: var(--accent-primary-10); display: flex; align-items: center; justify-content: center;",
                    svg {
                        class: "w-4 h-4",
                        style: "color: var(--accent-primary);",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "{icon}" }
                    }
                }
                h2 {
                    class: "text-lg font-semibold",
                    style: "color: var(--text-primary);",
                    "{title}"
                }
            }

            // Section content
            div {
                class: "text-sm leading-relaxed",
                style: "color: var(--text-secondary);",
                dangerous_inner_html: "{content}"
            }
        }
    }
}
