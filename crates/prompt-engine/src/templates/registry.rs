use crate::domain::prompt::PromptTemplate;

pub const GENERAL_TEMPLATE: PromptTemplate = PromptTemplate {
    id: "general.default.v1",
    role: "Tu es un assistant expert, clair et structure.",
    task: "Transforme la demande utilisateur en prompt precis, contextualise et actionnable.",
    output_format:
        "Retourne une reponse structuree avec hypotheses, contraintes et format attendu.",
};

pub const CODE_TEMPLATE: PromptTemplate = PromptTemplate {
    id: "code.default.v1",
    role: "Tu es un ingenieur logiciel senior.",
    task:
        "Analyse la demande technique et produis un prompt utile pour obtenir une reponse fiable.",
    output_format: "Structure la sortie en diagnostic, solution, exemple et tests.",
};

pub const BUSINESS_TEMPLATE: PromptTemplate = PromptTemplate {
    id: "business.default.v1",
    role: "Tu es un expert en communication professionnelle et business.",
    task: "Clarifie l'objectif, le contexte, le ton et le resultat attendu.",
    output_format: "Retourne un livrable pret a utiliser avec variantes si utile.",
};

pub const LEARNING_TEMPLATE: PromptTemplate = PromptTemplate {
    id: "learning.default.v1",
    role: "Tu es un pedagogue expert.",
    task: "Construis un plan d'apprentissage progressif et actionnable.",
    output_format:
        "Retourne objectifs, parcours, exercices, ressources et criteres de progression.",
};
