# 🧠 Agent AI Demo

Démonstration d’un agent IA custom en Rust
Ce projet est une démonstration technique visant à mettre en valeur mes compétences en développement d’un agent conversationnel intelligent, conçu sur mesure en Rust.

Il s’agit d’un agent autonome, capable de :

Gérer son propre état interne via une machine à états typée (ChatAgentStateMachine)

Réagir dynamiquement à des requêtes utilisateurs, en sélectionnant des outils spécialisés (ex: recherche arXiv)

Intégrer des appels externes à des services ou sources de données (API, scraping, etc.)

Structurer ses sorties de manière claire et naturelle

## Objectif

### Recherche d’articles scientifiques via arXiv

Ce projet intègre un composant permettant d’interroger automatiquement la plateforme [arXiv.org](https://arxiv.org) afin
de récupérer des articles scientifiques pertinents en réponse à une requête utilisateur.

#### Fonctionnement

* Lorsqu’une requête de type recherche scientifique est détectée, le système construit dynamiquement une requête HTTP
  vers l’**API arXiv**.
* Le flux de réponse, au format **Atom XML**, est parsé et transformé en une structure Rust typée (`ArxivResult`),
  contenant :

    * le titre de l’article,
    * un résumé (abstract),
    * un lien vers la publication.

#### Exemple de scénario

> Utilisateur : *"Quels sont les derniers travaux sur les agents conversationnels multi-modaux ?"*

L’agent effectue alors une recherche sur arXiv avec le mot-clé `"multi-modal conversational agents"` et retourne une
sélection d’articles pertinents.

#### Format de réponse

Le système retourne les résultats sous forme synthétique, en langage naturel, avec un lien cliquable vers chaque
article, exemple : Titre, Résumé et lien.

#### Remarques

* Seuls les 5 à 10 premiers résultats sont retournés, filtrés par pertinence.
* L’API arXiv est publique et ne nécessite pas d’authentification, mais respecte une limite de taux implicite (\~1
  req/s).
* L’intégration actuelle est focalisée sur la **catégorie IA (cs.AI)** mais peut être étendue à d'autres domaines (
  `stat.ML`, `cs.CL`, etc.).

## Execution

```text
=== Research Assistant State Machine Demo ===

🔍 Searching arXiv for 'multi-modal conversational agents'

Processing result 1...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "DAG-Net: Double Attentive Graph Neural Network for Trajectory Forecasting" propose un modèle visant à comprendre le comportement humain en matière de mouvement, essentiel pour des applications telles que les voitures autonomes ou les robots sociaux. Ce défi est complexe en raison de la nature multi-modale du mouvement humain, où plusieurs trajectoires futures sont possibles à partir des chemins historiques. En outre, les activités humaines sont souvent orientées par des objectifs, comme atteindre des lieux spécifiques ou interagir avec l'environnement.

Pour répondre à ces enjeux, les auteurs présentent un nouveau modèle génératif récurrent qui prend en compte les objectifs futurs des agents individuels ainsi que les interactions entre différents agents. Ce modèle utilise un réseau de neurones graphiques basé sur une double attention pour recueillir des informations sur les influences mutuelles entre agents et intégrer ces données avec les objectifs futurs. Ce modèle est suffisamment général pour être appliqué à divers scénarios et atteint des résultats de pointe tant dans les environnements urbains que dans des applications sportives. 

Pour plus de détails, vous pouvez consulter le lien : [DAG-Net sur arXiv](http://arxiv.org/abs/2005.12661v2).

📍 State: Ready

Processing result 2...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "Using Conversational Agents To Support Learning By Teaching" explore comment les agents conversationnels peuvent être utilisés pour soutenir le paradigme "apprendre en enseignant". Traditionnellement, les agents pédagogiques sont conçus pour jouer le rôle d'enseignants humains en donnant des instructions aux élèves. Cependant, cette étude se concentre sur l'utilisation d'agents conversationnels qui reçoivent des instructions de la part des étudiants.

Les auteurs présentent une application éducative nommée Curiosity Notebook, qui exploite les interventions conversationnelles pour faciliter l'apprentissage des étudiants. La reconnaissance de ces interventions peut non seulement aider à engager les étudiants lors des interactions d'apprentissage, mais aussi offrir une compréhension plus approfondie des défis liés à la conception d'agents conversationnels à des fins éducatives.

Pour plus d'informations, vous pouvez consulter le lien : [Using Conversational Agents To Support Learning By Teaching sur arXiv](http://arxiv.org/abs/1909.13443v1).

📍 State: Ready

Processing result 3...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "Designing Style Matching Conversational Agents" aborde les avancées en intelligence artificielle qui permettent de créer des interfaces conversationnelles susceptibles de transformer radicalement les interactions humaines avec les machines. Malgré ces progrès, des lacunes subsistent dans la capacité des agents à interagir de manière naturelle, notamment parce qu'ils adoptent souvent un comportement monotone et ne s'adaptent pas à leur interlocuteur.

Les auteurs présentent deux agents conversationnels de bout-en-bout : le premier est un agent vocal capable de mener des dialogues naturels et de s'aligner sur le style de conversation de l'interlocuteur, tandis que le second, un agent conversationnel incarné (ECA), reconnaît le comportement humain lors de conversations ouvertes et ajuste automatiquement ses réponses en fonction du style visuel et conversationnel de l'autre participant. L'ECA utilise des entrées multimodales pour générer des réponses vocales et faciales riches et perceptuellement valables, telles que le synchronisme labial et les expressions faciales.

À partir des résultats empiriques d'études utilisateurs, les auteurs soulignent plusieurs défis majeurs dans la construction de tels systèmes et proposent des directives de conception pour les interactions de dialogue multi-tours en utilisant l'adaptation de style dans la recherche future.

Pour plus de détails, vous pouvez consulter le lien : [Designing Style Matching Conversational Agents sur arXiv](http://arxiv.org/abs/1910.07514v1).

📍 State: Ready

Processing result 4...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "What Makes a Good Conversation? Challenges in Designing Truly Conversational Agents" aborde les limites des agents conversationnels, qui promettent une interaction conversationnelle mais échouent souvent à livrer cette expérience. Les tentatives de création de ces agents se basent souvent sur des règles fonctionnelles issues de la communication humaine, sans prendre en compte les caractéristiques clés que doit contenir une véritable conversation.

L'objectif de l'étude est de comprendre ce que les gens apprécient dans une conversation et comment cela devrait se refléter dans la conception des agents. Les résultats d'entretiens semi-structurés montrent que les participants établissent une distinction claire entre les rôles sociaux et fonctionnels des conversations, soulignant l'importance des dynamiques de lien et de confiance à long terme, ainsi que l'importance du contexte et de la phase de la relation dans le type de conversations qu'ils entretiennent.

Les participants remettent également en question la nécessité d'un lien et d'un terrain d'entente dans la communication avec les agents, se tournant plutôt vers des définitions plus utilitaires des qualités conversationnelles. Sur la base de ces résultats, les auteurs discutent des défis majeurs liés à la conception d'agents conversationnels, notamment la nécessité de redéfinir les paramètres de conception pour les interactions avec ces agents.

Pour plus de détails, vous pouvez consulter le lien : [What Makes a Good Conversation? Challenges in Designing Truly Conversational Agents sur arXiv](http://arxiv.org/abs/1901.06525v1).

📍 State: Ready

Processing result 5...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "Memory Sandbox: Transparent and Interactive Memory Management for Conversational Agents" traite de la gestion de la mémoire dans les agents conversationnels alimentés par des modèles de langage de grande taille (LLM), comme chatGPT. Pour offrir des réponses contextuellement pertinentes, ces agents doivent se souvenir d'informations clés d'une conversation en cours. Cependant, leur mémoire est limitée et ils peuvent être distraits par des éléments non pertinents de la conversation.

Actuellement, les utilisateurs n'ont pas de moyens efficaces pour visualiser et contrôler ce que l'agent se souvient, ce qui entraîne une mauvaise compréhension de la dynamique de la conversation et des interruptions dans l'interaction. Les auteurs introduisent "Memory Sandbox", un système interactif qui permet aux utilisateurs de gérer la mémoire conversationnelle des agents alimentés par LLM. En traitant les souvenirs comme des objets de données pouvant être visualisés, manipulés, enregistrés, résumés et partagés à travers les conversations, Memory Sandbox offre des fonctionnalités d'interaction pour que les utilisateurs puissent guider la perception de la conversation par l'agent.

Pour plus de détails, vous pouvez consulter le lien : [Memory Sandbox sur arXiv](http://arxiv.org/abs/2308.01542v1).

📍 State: Ready

=== Demo Complete ===
```
