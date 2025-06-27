# Agent AI Demo

### 🔎 Recherche d’articles scientifiques via arXiv

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

Le document intitulé "DAG-Net: Double Attentive Graph Neural Network for Trajectory Forecasting" 
traite de la compréhension du comportement de mouvement humain, ce qui est crucial pour des 
applications comme les voitures autonomes ou les robots sociaux, ainsi que pour tout environnement 
centré sur l'humain. Le défi réside dans le caractère multi-modal des mouvements humains, où 
plusieurs trajectoires futures sont possibles à partir d'un historique. De plus, ces mouvements 
sont souvent motivés par des objectifs, tels que atteindre un lieu particulier ou interagir avec 
l'environnement.

Pour aborder ces défis, les auteurs proposent un nouveau modèle génératif récurrent qui prend 
en compte à la fois les objectifs futurs des agents individuels et leurs interactions. Ce modèle 
utilise un réseau de neurones graphes à double attention pour recueillir des informations sur 
les influences mutuelles entre différents agents et les intégrer avec les données concernant 
les objectifs futurs possibles des agents. La méthode est suffisamment générale pour être appliquée
 à différents scénarios, et elle atteint des résultats à la pointe de la technologie dans des 
 environnements urbains ainsi que dans des applications sportives. 

Pour plus d'informations, vous pouvez consulter le lien suivant : 
[arXiv:2005.12661v2](http://arxiv.org/abs/2005.12661v2).

📍 State: Ready

Processing result 2...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "Using Conversational Agents To Support Learning By Teaching" explore l'utilisation des agents conversationnels pour soutenir et faciliter l'apprentissage. Contrairement aux agents pédagogiques classiques qui agissent comme des enseignants en fournissant des instructions aux étudiants, cette étude examine comment des agents conversationnels peuvent soutenir le paradigme de "l'apprentissage par l'enseignement", où l'agent reçoit des instructions des étudiants.

Les auteurs introduisent une application éducative appelée Curiosity Notebook, qui exploite les interventions conversationnelles pour faciliter l'apprentissage des étudiants. La reconnaissance de ces interventions permet non seulement d'engager les étudiants dans des interactions d'apprentissage, mais aussi d'offrir une meilleure compréhension des complexités liées à la conception d'agents conversationnels pour des fins éducatives.

Pour plus de détails, vous pouvez consulter le lien suivant : [arXiv:1909.13443v1](http://arxiv.org/abs/1909.13443v1).

📍 State: Ready

Processing result 3...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "Designing Style Matching Conversational Agents" traite des avancées en intelligence machine qui ont permis le développement d'interfaces conversationnelles capable de transformer les interactions humaines avec les machines. Malgré ces progrès, des lacunes subsistent dans la capacité de ces agents à interagir de manière naturelle, notamment en raison de leur comportement souvent monotone et de leur manque d'adaptation aux partenaires de conversation.

Les auteurs ont développé deux agents conversationnels de bout en bout : un agent vocal capable de maintenir un dialogue naturel sur plusieurs tours tout en s'alignant sur le style conversationnel de l'interlocuteur, et un agent conversationnel incarné (ECA) qui reconnaît le comportement humain lors de conversations ouvertes et ajuste automatiquement ses réponses au style visuel et conversationnel de l'autre partie. Cet agent incarné utilise des entrées multimodales pour générer des réponses vocales et faciales riches et perceptivement valables (comme le synchronisme labial et les expressions) pendant la conversation.

À partir de résultats empiriques d'études utilisateurs, les auteurs identifient des défis majeurs dans la création de tels systèmes et proposent des lignes directrices pour les interactions de dialogue en plusieurs tours utilisant l'adaptation de style pour les recherches futures.

Pour plus de précisions, vous pouvez consulter le lien suivant : [arXiv:1910.07514v1](http://arxiv.org/abs/1910.07514v1).

📍 State: Ready

Processing result 4...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "What Makes a Good Conversation? Challenges in Designing Truly Conversational Agents" aborde les attentes envers les agents conversationnels qui promettent une interaction verbale, mais qui ne parviennent pas à répondre à cette promesse. Bien que des efforts aient été faits pour reproduire des règles fonctionnelles du discours humain, il est souvent négligé des caractéristiques clés que la conversation doit englober.

L'objectif de cette étude est de comprendre ce que les gens apprécient dans une conversation et comment cela devrait se traduire dans le comportement des agents. À travers une série d'entretiens semi-structurés, les résultats montrent que les utilisateurs établissent une distinction claire entre les rôles social et fonctionnel de la conversation. Ils soulignent l'importance des dynamiques à long terme, du lien et de la confiance, ainsi que du contexte et du stade de la relation dans les types de conversations qu'ils ont.

Les participants remettent fondamentalement en question la nécessité de lien et de terrain commun dans la communication avec les agents, se tournant vers des définitions plus utilitaires des qualités conversationnelles. Sur la base de ces résultats, les auteurs discutent des principaux défis dans la conception d'agents conversationnels, notamment la nécessité de redéfinir les paramètres de conception pour les interactions avec ces agents.

Pour plus de détails, vous pouvez consulter le lien suivant : [arXiv:1901.06525v1](http://arxiv.org/abs/1901.06525v1).

📍 State: Ready

Processing result 5...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "Memory Sandbox: Transparent and Interactive Memory Management for Conversational Agents" aborde les défis liés à la gestion de la mémoire dans les agents conversationnels basés sur de grands modèles de langage (LLM), tels que chatGPT. Ces agents doivent mémoriser des informations clés d'une conversation en cours pour fournir des réponses contextuellement pertinentes, mais ils disposent d'une mémoire limitée et peuvent être distraits par des éléments non pertinents.

Actuellement, les utilisateurs n'ont pas de moyens pour visualiser et contrôler ce que l'agent retient, ce qui entraîne une mauvaise compréhension (modèle mental) et des ruptures dans la conversation. Les auteurs proposent donc Memory Sandbox, un système interactif qui permet aux utilisateurs de gérer la mémoire conversationnelle de ces agents. En traitant les souvenirs comme des objets de données pouvant être visualisés, manipulés, enregistrés, résumés et partagés entre les conversations, Memory Sandbox offre aux utilisateurs des moyens d'interagir avec la mémoire de l'agent, leur permettant de mieux gérer la façon dont l'agent "perçoit" la conversation.

Pour plus de détails, vous pouvez consulter le lien suivant : [arXiv:2308.01542v1](http://arxiv.org/abs/2308.01542v1).

📍 State: Ready

=== Demo Complete ===
```