#le funzioni implementano un preprocessing per ciascuno dei dataset
# serve per distinguere i dati categorici da quelli che non lo sono

def sepsis_dset():
    cat_cols = ['sex_woman', 'diagnosis_0EC_1M_2_AC',
               'Group', 'Mortality']
    dict_type = {col: "category" for col in cat_cols}
    name_csv = "journal.pone.0148699_S1_Text_Sepsis_SIRS_EDITED.csv"
    return dict_type, name_csv


def neuroblastoma_dset():
    cat_cols = ['age', 'sex', 'site', 'stage', 'risk',
       'autologous_stem_cell_transplantation', 'radiation',
       'degree_of_differentiation', 'UH_or_FH', 'MYCN_status ',
       'surgical_methods', 'outcome']
    dict_type = {col: "category" for col in cat_cols}
    name_csv = "10_7717_peerj_5665_dataYM2018_neuroblastoma.csv"
    return dict_type, name_csv

# la variabile da predirre è la penultima
def depression_heart_f_dset():
    cat_cols = ['Male', 'Etiology', 'Death', 'Hospitalized']
    
    dict_type ={col: "category" for col in cat_cols}
    name_csv = "journal.pone.0158570_S2File_depression_heart_failure.csv"
    return dict_type, name_csv 



#la variabile da predirre è la prima

def cardiac_arrest_spain_dset():
    
    cat_cols = [ 'Exitus', 'sex_woman', 'Endotracheal_intubation',
               'Functional_status', 'Asystole', 'Bystander',
               'Cardiogenic', 'Cardiac_arrest_at_home' ] 
    
    
    
    dict_type = {col: "category" for col in cat_cols}
    name_csv = "journal.pone.0175818_S1Dataset_Spain_cardiac_arrest_EDITED.csv"
    return dict_type, name_csv 



def diabetes_type1_dset():
    cat_cols = ['sex_0man_1woman', 'insulin_regimen_binary']

    dict_type = {col: "category" for col in cat_cols}
    
    name_csv = "Takashi2019_diabetes_type1_dataset_preprocessed.csv"
    
    return dict_type, name_csv 
