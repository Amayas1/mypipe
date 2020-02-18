//KHELFOUNE Amayas khelfouneamayas@gmail.com

clap de caisse externe ;
utilisez std :: process :: Stdio;
utilisez clap :: {App, Arg};
utilisez std :: process :: Command;


fn  main () {
    let matches = App :: new ( "My Pipe" )
        . version ( "version: 1.0" )
        . auteur ( "Amayas KHELFOUNE <khelfouneamayas@gmail.com>" )
        . arg (
            Arg :: with_name ( "in" )
                . long ( "entrée" )
                . court ( "in" )
                . takes_value ( true )
                . requis ( vrai ),
        )
        . arg (
            Arg :: with_name ( "out" )
                . long ( "sortie" )
                . court ( "out" )
                . takes_value ( true )
                . requis ( vrai ),
        )
        . get_matches ();

    laissez input = correspond. value_of ( "in" ). dérouler ();
    laissez output = correspond. value_of ( "out" ). dérouler ();

    let input_args = Command :: new (entrée)
        . stdout (Stdio :: canalisé ())
        . spawn ()
        . attend ( "Erreur de chargement d'entrée" );

    
    
    let output_args = Command :: new (sortie)
        . stdin (input_args.stdout. dépliage ())
        . sortie ()
        . attend ( "Erreur de chargement de sortie" );

    // résultat
    println! ( "{}" , String :: from_utf8_lossy ( & (output_args) .stdout));
}