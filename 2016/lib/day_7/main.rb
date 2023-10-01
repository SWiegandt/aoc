require_relative '../util'

class Day7
  def initialize
    @input = Util.read_input(7).split("\n")
  end

  def main
    puts(problem_one)
    puts(problem_two)
  end

  def abba?(str)
    (0..str.length - 3).each do |i|
      return true if str[i] == str[i + 3] && str[i + 1] == str[i + 2] && str[i] != str[i + 1]
    end

    false
  end

  def tls?(address)
    address.scan(/\[(.*?)\]/).each do |hypernet|
      return false if abba?(hypernet[0])
    end

    abba?(address)
  end

  def ssl?(address)
    abas = Set.new
    babs = Set.new

    address.split(/[\[\]]/).each_with_index do |str, i|
      (0..str.length - 2).each do |j|
        if str[j] == str[j + 2] && str[j] != str[j + 1]
          if i.even?
            abas.add(str[j..j + 2])
          else
            babs.add(str[j + 1] + str[j] + str[j + 1])
          end
        end
      end
    end

    abas.intersect?(babs)
  end

  def problem_one
    @input.count { |address| tls?(address) }
  end

  def problem_two
    @input.count { |address| ssl?(address) }
  end
end

Day7.new.main
